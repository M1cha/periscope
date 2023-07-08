#include "proc.hpp"
#include "ipc.h"
#include "main_gui.hpp"
#include "config.hpp"

bool is_running(u64 program) {
	u64 pid = 0;
	if (R_FAILED(pmdmntGetProcessId(&pid, program))) {
		return false;
	}
	return pid > 0;
}

void start_prog(u64 program) {
	NcmProgramLocation loc = {.program_id = program, .storageID = NcmStorageId_None};
	u64 pid = 0;
	pmshellLaunchProgram(0, &loc, &pid);
}

void kill_prog(u64 program) {
	pmshellTerminateProgram(program);
}

bool restart_listener(u64 keys, Config *config, bool *error) {
	if (keys & HidNpadButton_A) {
		u64 prog_id = 0x420000000005C09EULL;
		if (is_running(prog_id)) {
			kill_prog(prog_id);
		}
		start_prog(prog_id);
		svcSleepThread(200000000LL);
		tsl::hlp::doWithSmSession([config, error]() {
			ipc_init();
			if (!ipc_running() || ipc_getver() != IPCVER) {
				*error = true;
			} else {
				tsl::changeTo<MainGui>(config);
			}
		});
		return true;
	}
	return false;
}
