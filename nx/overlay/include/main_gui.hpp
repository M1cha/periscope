#ifndef MAIN_GUI_HPP_
#define MAIN_GUI_HPP_
#include "config.hpp"
#include <tesla.hpp>

class MainGui : public tsl::Gui {
  public:
	MainGui(Config *config);
	virtual tsl::elm::Element *createUI() override;

  private:
	Config *cfg;
	tsl::elm::List *list;
};

#endif // MAIN_GUI_HPP_
