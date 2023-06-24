#ifndef SERVER_H_
#define SERVER_H_
#include <switch.h>

void server_setup(void);
int accept_conn(void);
int read_msg(char *buf, int size);
int send_msg(char *buf, int size);
void server_takedown(void);
int build_payload(u32 buttons, HidAnalogStickState l, HidAnalogStickState r, char *buf);

#endif // SERVER_H_