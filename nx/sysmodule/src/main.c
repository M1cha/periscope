#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <switch.h>

#include "server.h"

#define INNER_HEAP_SIZE 0x80000

u32 __nx_applet_type = AppletType_None;
u32 __nx_fs_num_sessions = 1;
void __libnx_initheap(void) {
	static u8 inner_heap[INNER_HEAP_SIZE];
	extern void *fake_heap_start;
	extern void *fake_heap_end;

	// Configure the newlib heap.
	fake_heap_start = inner_heap;
	fake_heap_end = inner_heap + sizeof(inner_heap);
}

// Service initialization.
void __appInit(void) {
	Result rc;

	// Open a service manager session.
	rc = smInitialize();
	if (R_FAILED(rc))
		diagAbortWithResult(MAKERESULT(Module_Libnx, LibnxError_InitFail_SM));

	// Retrieve the current version of Horizon OS.
	rc = setsysInitialize();
	if (R_SUCCEEDED(rc)) {
		SetSysFirmwareVersion fw;
		rc = setsysGetFirmwareVersion(&fw);
		if (R_SUCCEEDED(rc))
			hosversionSet(MAKEHOSVERSION(fw.major, fw.minor, fw.micro));
		setsysExit();
	}
	rc = hidInitialize();
	if (R_FAILED(rc))
		diagAbortWithResult(MAKERESULT(Module_Libnx, LibnxError_InitFail_HID));

	// Disable this if you don't want to use the filesystem.
	/*rc = fsInitialize();
	if (R_FAILED(rc))
	    diagAbortWithResult(MAKERESULT(Module_Libnx, LibnxError_InitFail_FS));

	// Disable this if you don't want to use the SD card filesystem.
	fsdevMountSdmc();*/

	// Add other services you want to use here.

	// Close the service manager session.
}

void __appExit(void) {
	hidExit();
	smExit();
}

int main(int argc, char *argv[]) {
	padConfigureInput(1, HidNpadStyleSet_NpadStandard);
	PadState pad;
	HidAnalogStickState l, r;
	padInitializeDefault(&pad);

	static const SocketInitConfig socketInitConfig = {
	    .bsdsockets_version = 1,
	    .tcp_tx_buf_size = 1024,
	    .tcp_rx_buf_size = 256,
	    .tcp_tx_buf_max_size = 0,
	    .tcp_rx_buf_max_size = 0,
	    .udp_tx_buf_size = 0x2400,
	    .udp_rx_buf_size = 0xA500,
	    .sb_efficiency = 4,
	    .num_bsd_sessions = 3,
	    .bsd_service_type = BsdServiceType_User,
	};
	socketInitialize(&socketInitConfig);
	server_setup();

	char client_msg[10];
	char payload[128] = {0};
	u64 down;
	u32 to_send;
	while (appletMainLoop()) {
		if (accept_conn() < 0) {
			server_takedown();
			server_setup();
			continue;
		}
		while (true) {
			if (read_msg(client_msg, 10) < 0) {
				break;
			}
			padUpdate(&pad);
			down = padGetButtons(&pad);
			l = padGetStickPos(&pad, 0);
			r = padGetStickPos(&pad, 1);
			to_send = (u32)down & 0xF00FFFF;
			int len = build_payload(to_send, l, r, payload);
			if (send_msg(payload, len) < 0) {
				break;
			}
		}
	}
	socketExit();
	return 0;
}
