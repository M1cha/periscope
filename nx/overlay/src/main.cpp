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
	Config *config;
	virtual void initServices() override {
		pmshellInitialize();
		fsdevMountSdmc();
		if (!ipc_running()) {
			problem2 = true;
		} else {
			ipc_init();
			if (ipc_getver() != IPCVER) {
				problem = true;
			}
		}
	}
	virtual void exitServices() override {
		pmshellExit();
		fsdevUnmountAll();
		ipc_exit();
	}

	virtual std::unique_ptr<tsl::Gui> loadInitialGui() override {
		config = new Config();
		if (problem2)
			return initially<ErrorGui>("sys-scope is not running!", config, true);
		if (problem)
			return initially<ErrorGui>("overlay and sysmodule versions don't match!", config);
		return initially<MainGui>(config);
	}
};

int main(int argc, char **argv) {
	return tsl::loop<PeriscopeOverlay>(argc, argv);
}
