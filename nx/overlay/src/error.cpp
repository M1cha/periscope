#include "error.hpp"
#include <string>
#include <tesla.hpp>

ErrorGui::ErrorGui(std::string msg) {
	message = msg;
}

tsl::elm::Element *ErrorGui::createUI() {
	auto frame = new tsl::elm::OverlayFrame("periscope", "0.1.0");
	auto list = new tsl::elm::List();
	auto it = new tsl::elm::ListItem(message);
	list->addItem(it);
	frame->setContent(list);
	return frame;
}
