#include "svc.h"
#include "ipc.h"
#include "ipc_server.h"
#include "server.h"

IpcServer server;

void service_scope_init() {
	ipcServerInit(&server, "scope", 2);
}

void service_scope_exit() {
	ipcServerExit(&server);
}

bool ipc_running = true;

void service_scope_stop() {
	ipc_running = false;
}

Result service_handler(void *arg, const IpcServerRequest *r, u8 *out_data, size_t *out_size) {
	bool *enabled_controllers = (bool *)arg;
	enabled_controllers[0] = true;
	switch (r->data.cmdId) {
		case SC_GETVER:
			enabled_controllers[0] = !enabled_controllers[0];
			*out_size = sizeof(int);
			*(int *)out_data = IPCVER;
			break;
		case SC_GETIP:
			*out_size = sizeof(int);
			*(int *)out_data = server_ip();
			break;
		case SC_ENABLECONTROLLER:
			if (r->data.size == sizeof(int)) {
				enabled_controllers[*(int *)r->data.ptr] = true;
			}
			break;
		case SC_DISABLECONTROLLER:
			if (r->data.size == sizeof(int)) {
				enabled_controllers[*(int *)r->data.ptr] = false;
			}
			break;
	}
	return 0; // idfk
}

void service_scope_func(void *enabled_controllers) {
	while (ipc_running) {
		if (ipcServerProcess(&server, service_handler, enabled_controllers) == KERNELRESULT(Cancelled)) {
			break;
		}
	}
}
