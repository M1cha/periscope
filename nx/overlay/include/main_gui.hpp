#ifndef MAIN_GUI_HPP_
#define MAIN_GUI_HPP_
#include "config.hpp"
#include <tesla.hpp>

class MainGui : public tsl::Gui {
  public:
	MainGui();
	virtual tsl::elm::Element *createUI() override;
	virtual void update() override;
	virtual bool handleInput(
	    u64 keysDown, u64 keysHeld, const HidTouchState &touchPos, HidAnalogStickState joyStickPosLeft, HidAnalogStickState joyStickPosRight) override;

  private:
	Config cfg;
	tsl::elm::List *list;
};

#endif // MAIN_GUI_HPP_
