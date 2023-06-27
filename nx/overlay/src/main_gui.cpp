#include "main_gui.hpp"
#include "config.hpp"
#include "ipc.h"
#include <string>
#include <tesla.hpp>

MainGui::MainGui() {
	cfg = Config();
}

tsl::elm::Element *MainGui::createUI() {
	auto frame = new tsl::elm::OverlayFrame("periscope", "0.1.0");
	list = new tsl::elm::List();
	char *ip = ipc_getip();
	auto ip_el = new tsl::elm::ListItem("IP: ", ip);
	list->addItem(ip_el);
	auto multitoggle = new tsl::elm::ToggleListItem("Multi-controller", cfg.multicap(), "Enabled", "Disabled");
	multitoggle->setStateChangedListener([this](bool state) { this->cfg.set_multicap(state); });
	list->addItem(multitoggle);
	auto header = new tsl::elm::CategoryHeader("Enabled controllers", true);
	list->addItem(header);
	std::string player_text = "Player 1";
	for (int i = 0; i < 8; i++) {
		auto el = new tsl::elm::ToggleListItem(player_text, this->cfg.enabled(i), "On", "");
		el->setStateChangedListener([this, i](bool state) { this->cfg.set_enabled(i, state); });
		list->addItem(el);
		player_text[7]++;
	}
	frame->setContent(list);
	return frame;
}

void MainGui::update() {
	for (int i = 3; i < 11; i++) {
		static_cast<tsl::elm::ToggleListItem *>(list->getItemAtIndex(i))->setState(cfg.enabled(i - 3));
	}
}

bool MainGui::handleInput(
    u64 keysDown, u64 keysHeld, const HidTouchState &touchPos, HidAnalogStickState joyStickPosLeft, HidAnalogStickState joyStickPosRight) {
	return false;
}
