#include "main_gui.hpp"
#include "proc.hpp"
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
	auto header2 = new tsl::elm::CategoryHeader("Restart", true);
	list->addItem(header2);
	auto restart_el = new tsl::elm::ListItem("Restart sys-scope");
	restart_el->setClickListener([this](u64 keys) {
		bool error;
		return restart_listener(keys, this->cfg, &error);
	});
	list->addItem(restart_el);
	frame->setContent(list);
	return frame;
}

bool MainGui::handleInput(u64 keysDown, u64 keysHeld, const HidTouchState &touchPos, HidAnalogStickState leftJoyStick, HidAnalogStickState rightJoyStick) {
	if (keysDown & HidNpadButton_B) {
		// this is so cringe but i don't Care
		tsl::goBack();
		tsl::goBack();
		return true;
	}
	return false;
}
