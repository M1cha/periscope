#ifndef IPC_H_
#define IPC_H_
#include <switch.h>

enum ScopeCmd {
	SC_ENABLECONTROLLER,
	SC_DISABLECONTROLLER,
	SC_ENABLEMULTICAP,
	SC_DISABLEMULTICAP,
	SC_GETIP,

	SC_GETVER = 1000,
};

#ifdef __cplusplus
extern "C" {
#endif

Result ipc_init();
int ipc_getver();
char *ipc_getip();
void ipc_exit();
#ifdef __cplusplus
}
#endif

const int IPCVER = 1;

#endif // IPC_H_
