#define TESLA_INIT_IMPL // If you have more than one file using the tesla header, only define this in the main one
#include "config.hpp"
#include "error.hpp"
#include "ipc.h"
#include "main_gui.hpp"
#include <tesla.hpp> // The Tesla Header

class PeriscopeOverlay : public tsl::Overlay {
  public:
	bool problem = false;
	bool problem2 = false;
	// libtesla already initialized fs, hid, pl, pmdmnt, hid:sys and set:sys
	virtual void initServices() override {
		fsdevMountSdmc();
		if (!ipc_running()) {
			problem2 = true;
		} else {
			ipc_init();
			if (ipc_getver() != IPCVER) {
				problem = true;
			}
		}
	} // Called at the start to initialize all services necessary for this Overlay
	virtual void exitServices() override {
		fsdevUnmountAll();
		ipc_exit();
	} // Callet at the end to clean up all services previously initialized

	virtual void onShow() override {} // Called before overlay wants to change from invisible to visible state
	virtual void onHide() override {} // Called before overlay wants to change from visible to invisible state

	virtual std::unique_ptr<tsl::Gui> loadInitialGui() override {
		if (problem2)
			return initially<ErrorGui>("sys-scope is not running!");
		if (problem)
			return initially<ErrorGui>("overlay and sysmodule versions don't match!");
		return initially<MainGui>(); // Initial Gui to load. It's possible to pass arguments to it's constructor like this
	}
};

int main(int argc, char **argv) {
	return tsl::loop<PeriscopeOverlay>(argc, argv);
}
