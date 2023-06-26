#define TESLA_INIT_IMPL // If you have more than one file using the tesla header, only define this in the main one
#include "config.hpp"
#include "ipc.h"
#include <string>
#include <tesla.hpp> // The Tesla Header

class ProblemGui : public tsl::Gui {
  public:
	ProblemGui() {}
	virtual tsl::elm::Element *createUI() override {
		auto frame = new tsl::elm::OverlayFrame("periscope", "0.1.0");
		auto list = new tsl::elm::List();
		auto it = new tsl::elm::ListItem("Overlay and sysmodule versions don't match!");
		list->addItem(it);
		frame->setContent(list);
		return frame;
	}
};
class Problem2Gui : public tsl::Gui {
  public:
	Problem2Gui() {}
	virtual tsl::elm::Element *createUI() override {
		auto frame = new tsl::elm::OverlayFrame("periscope", "0.1.0");
		auto list = new tsl::elm::List();
		auto it = new tsl::elm::ListItem("sys-scope not running!");
		list->addItem(it);
		frame->setContent(list);
		return frame;
	}
};

class PeriscopeGui : public tsl::Gui {
  public:
	PeriscopeGui() {
		cfg = Config();
	}

	// Called when this Gui gets loaded to create the UI
	// Allocate all elements on the heap. libtesla will make sure to clean them up when not needed anymore
	virtual tsl::elm::Element *createUI() override {
		// A OverlayFrame is the base element every overlay consists of. This will draw the default Title and Subtitle.
		// If you need more information in the header or want to change it's look, use a HeaderOverlayFrame.
		auto frame = new tsl::elm::OverlayFrame("periscope", "0.1.0");

		// A list that can contain sub elements and handles scrolling
		auto list = new tsl::elm::List();
		char *ip = ipc_getip();
		auto ip_el = new tsl::elm::ListItem("IP: ", ip);
		list->addItem(ip_el);
		auto multitoggle = new tsl::elm::ToggleListItem("Multi-controller", false, "Enabled", "Disabled");
		multitoggle->setStateChangedListener([this](bool state) { this->cfg.multi = state; });
		list->addItem(multitoggle);
		auto header = new tsl::elm::CategoryHeader("Enabled controllers");
		list->addItem(header);

		// Create and add a new list item to the list
		std::string player_text = "Player 1";
		for (int i = 0; i < 8; i++) {
			auto el = new tsl::elm::ToggleListItem(player_text, this->cfg.enabled(i), "On", "");
			el->setStateChangedListener([this, i, list](bool state) {
				this->cfg.set_enabled(i, state);
				for (int i = 3; i < 11; i++) {
					static_cast<tsl::elm::ToggleListItem *>(list->getItemAtIndex(i))->setState(this->cfg.enabled(i - 3));
				}
			});
			list->addItem(el);
			player_text[7]++;
		}

		// Add the list to the frame for it to be drawn
		frame->setContent(list);

		// Return the frame to have it become the top level element of this Gui
		return frame;
	}

	// Called once every frame to update values
	virtual void update() override {}

	// Called once every frame to handle inputs not handled by other UI elements
	virtual bool handleInput(
	    u64 keysDown, u64 keysHeld, const HidTouchState &touchPos, HidAnalogStickState joyStickPosLeft, HidAnalogStickState joyStickPosRight) override {
		return false; // Return true here to signal the inputs have been consumed
	}

  private:
	Config cfg;
};

class PeriscopeOverlay : public tsl::Overlay {
  public:
	bool problem = false;
	bool problem2 = false;
	// libtesla already initialized fs, hid, pl, pmdmnt, hid:sys and set:sys
	virtual void initServices() override {
		fsdevMountSdmc();
		Result rc = ipc_init();
		if (R_MODULE(rc) == Module_Kernel && R_VALUE(rc) == KERNELRESULT(NotFound)) {
			problem2 = true;
		};
		if (ipc_getver() != IPCVER) {
			problem = true;
		}
	} // Called at the start to initialize all services necessary for this Overlay
	virtual void exitServices() override {
		ipc_exit();
	} // Callet at the end to clean up all services previously initialized

	virtual void onShow() override {} // Called before overlay wants to change from invisible to visible state
	virtual void onHide() override {} // Called before overlay wants to change from visible to invisible state

	virtual std::unique_ptr<tsl::Gui> loadInitialGui() override {
		if (problem2)
			return initially<Problem2Gui>();
		if (problem)
			return initially<ProblemGui>();
		return initially<PeriscopeGui>(); // Initial Gui to load. It's possible to pass arguments to it's constructor like this
	}
};

int main(int argc, char **argv) {
	return tsl::loop<PeriscopeOverlay>(argc, argv);
}
