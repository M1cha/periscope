#include "player_gui.hpp"
#include "config.hpp"
#include <tesla.hpp>

PlayerGui::PlayerGui(Config *config) {
	cfg = config;
}

tsl::elm::Element *PlayerGui::createUI() {
	auto frame = new tsl::elm::OverlayFrame("periscope", "1.0.0");
	list = new tsl::elm::List();

	auto multitoggle = new tsl::elm::ToggleListItem("Multi-controller", cfg->multicap(), "Enabled", "Disabled");
	multitoggle->setStateChangedListener([this](bool state) { this->cfg->set_multicap(state); });
	list->addItem(multitoggle);

	auto header = new tsl::elm::CategoryHeader("Enabled controllers", true);
	list->addItem(header);

	std::string player_text = "Player 1";
	for (int i = 0; i < 8; i++) {
		auto el = new tsl::elm::ToggleListItem(player_text, cfg->enabled(i), "On", "");
		el->setStateChangedListener([this, i](bool state) { this->cfg->set_enabled(i, state); });
		list->addItem(el);
		player_text[7]++;
	}
	frame->setContent(list);
	return frame;
}

void PlayerGui::update() {
	for (int i = 2; i < 10; i++) {
		auto a = static_cast<tsl::elm::ToggleListItem *>(list->getItemAtIndex(i));
		if (a) {
			a->setState(cfg->enabled(i - 2));
		}
	}
}
