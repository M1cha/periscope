#ifndef SVC_H_
#define SVC_H_
#include <switch.h>

void service_scope_init();
void service_scope_stop();
void service_scope_exit();
void service_scope_func(void *enabled_controllers);

typedef struct {
	bool pads_enabled[8];
	bool multicap;
} rt_config;

#endif // SVC_H_
