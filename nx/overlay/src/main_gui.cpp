#include "main_gui.hpp"
#include "config.hpp"
#include "ipc.h"
#include "player_gui.hpp"
#include <string>
#include <switch.h>
#include <tesla.hpp>

MainGui::MainGui(Config *config) {
	cfg = config;
}

tsl::elm::Element *MainGui::createUI() {
	auto frame = new tsl::elm::OverlayFrame("periscope", "0.1.0");
	list = new tsl::elm::List();
	char *ip = ipc_getip();
	auto ip_el = new tsl::elm::ListItem("IP: ", ip);
	list->addItem(ip_el);
	auto header = new tsl::elm::CategoryHeader("Configuration", true);
	list->addItem(header);
	auto player_el = new tsl::elm::ListItem("Controllers");
	Config *c = cfg;
	player_el->setClickListener([c](u64 keys) {
		if (keys & HidNpadButton_A) {
			tsl::changeTo<PlayerGui>(c);
			return true;
		}
		return false;
	});
	list->addItem(player_el);
	frame->setContent(list);
	return frame;
}

