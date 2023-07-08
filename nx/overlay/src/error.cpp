#include "error.hpp"
#include "ipc.h"
#include "proc.hpp"
#include "config.hpp"
#include "main_gui.hpp"
#include <string>
#include <tesla.hpp>

ErrorGui::ErrorGui(std::string msg, Config *cfg, bool can_start) {
	config = cfg;
	message = msg;
	show_start = can_start;
	error = false;
}

tsl::elm::Element *ErrorGui::createUI() {
	auto frame = new tsl::elm::OverlayFrame("periscope", "1.1.0");
	auto list = new tsl::elm::List();
	auto it = new tsl::elm::ListItem(message);
	list->addItem(it);
	if (show_start) {
		auto it2 = new tsl::elm::ListItem("Start sys-scope");
		start_item = it2;
		it2->setClickListener([this](u64 keys) {
			return restart_listener(keys, this->config, &this->error);
		});
		auto header = new tsl::elm::CategoryHeader("Start");
		list->addItem(header);
		list->addItem(it2);
	}
	frame->setContent(list);
	return frame;
}

void ErrorGui::update() {
	if (error) {
		start_item->setText("Failed to start sys-scope");
		error = false;
	}
}
