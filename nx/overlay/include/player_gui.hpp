#ifndef PLAYER_GUI_HPP_
#define PLAYER_GUI_HPP_
#include "config.hpp"
#include <tesla.hpp>

class PlayerGui : public tsl::Gui {
  public:
	PlayerGui(Config *config);
	virtual tsl::elm::Element *createUI() override;
	virtual void update() override;

  private:
	Config *cfg;
	tsl::elm::List *list;
};

#endif // PLAYER_GUI_HPP_
