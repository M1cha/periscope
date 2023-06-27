#include <arpa/inet.h>
#include <stdio.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>

#include <switch.h>

int client_fd, server_fd;
struct sockaddr_in server_addr;

void server_setup(void) {
	server_fd = socket(AF_INET, SOCK_STREAM, 0);
	server_addr.sin_family = AF_INET;
	server_addr.sin_port = htons(2579);
	server_addr.sin_addr.s_addr = gethostid();
	while (bind(server_fd, (struct sockaddr *)&server_addr, sizeof(server_addr)) < 0) {
		svcSleepThread(5e+8L); // 500ms
	}
	listen(server_fd, 1);
}

int server_ip() {
	socklen_t len = sizeof(struct sockaddr_in);
	getsockname(server_fd, (struct sockaddr *)&server_addr, &len);
	return server_addr.sin_addr.s_addr;
}

int accept_conn(void) {
	return (client_fd = accept(server_fd, NULL, NULL));
}

int read_msg(char *buf, int size) {
	memset(buf, 0, size);
	return recv(client_fd, buf, size, 0);
}

int send_msg(char *buf, int size) {
	return send(client_fd, buf, size, 0);
}

void server_takedown(void) {
	if (client_fd > 0) {
		close(client_fd);
	}
	close(server_fd);
}

int build_payload(int id, u32 buttons, HidAnalogStickState l, HidAnalogStickState r, char *buf) {
	memset(buf, 0, 99);
	return snprintf(buf, 128, "{\"id\":%d,\"bs\":%i,\"ls\":{\"x\":%d,\"y\":%d},\"rs\":{\"x\":%d,\"y\":%d}},", id, buttons, l.x, l.y, r.x, r.y);
}
