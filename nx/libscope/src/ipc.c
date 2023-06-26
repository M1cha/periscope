#include "ipc.h"
#include <stdio.h>
#include <switch.h>

Service scope;

Result ipc_init() {
	Handle h = 0;
	Result rc = svcConnectToNamedPort(&h, "scope");
	if (R_SUCCEEDED(rc)) {
		serviceCreate(&scope, h);
	}
	return rc;
}

int ipc_getver() {
	int ver = 0;
	serviceDispatchOut(&scope, SC_GETVER, ver);
	return ver;
}

char ipa[16];

char *ipc_getip() {
	unsigned char ip[4];
	serviceDispatchOut(&scope, SC_GETIP, ip);
	snprintf(ipa, 16, "%d.%d.%d.%d", ip[0], ip[1], ip[2], ip[3]);
	return ipa;
}

void ipc_exit() {
	serviceClose(&scope);
}
