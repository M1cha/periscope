# periscope
homebrew solution for nintendo switch input display

this is heavily inspired by openjoystickdisplay / OJDS-NX, which is implemented almost the same way. however, openjoystickdisplay has been abandonware for multiple years, and the OJDS-NX project was recently deleted.
therefore, i have reimplemented both sides of the equation, specifically for switch, rather than the ojd general input setup.

### current features
* switch sysmodule that grabs inputs
* desktop viewer program that connects to the switch and pulls inputs from the sysmodule
* support for 1 player (pro controller, dual joy-con, or single joy-con (*i think*))
* custom skin support
* configuration dialog box

### planned features
* a default skin to distribute with this that actually looks ok
* better desktop configuration (e.g. make sure all the fields are actually valid before trying to start)
* tesla overlay to get IP address as well as configure sysmodule (?)
* support for gyro
* support for accelerometer (likely difficult)
* support for multiple players
* support for capturing a specific player out of many

### skin configuration
each skin lives in its own directory in the config folder (e.g. `~/.config/periscope/` on linux, idk other platforms). inside this directory
is a `skin.toml` file and every image you need for the skin. you need at least a background image and one other, to use for buttons and joysticks.
each button and joystick can be a different image, if you want. each also has a `pos`, which is where on the screen it will be displayed. x increases
to the right and y towards the bottom, starting at the top left. joysticks also have a `range`, which is the radius of the circle that the joystick
can travel in on the viewer.

the list of buttons is:

a, b, x, y, plus, minus, zl, zr, l, r, up, down, left, right, ls, rs, lsl, lsr, rsl, rsr

which should be fairly self-explanatory except the last few, which are left and right joycons' SL and SR buttons.

skin.toml looks like this:
```toml
background = "bg.png" # file bg.png in the same directory as this file
buttons.a = {image = "button.png", pos = {x = 50, y = 50}}
buttons.b = {image = "button.png", pos = {x = 40, y = 60}}
# repeat for each button ...

ls = {image = "stick.png", pos = {x = 100, y = 100}, range = 50}
# repeat for right stick
```

if you leave a button or joystick unconfigured, it simply will not be rendered.

### building
the viewer lives in `desktop/`, and is written in rust. you need to install cargo and rust to build it, then to build, `cargo b --release` and to run `cargo r --release`.

binary will be at target/release/periscope

the sysmodule lives in `nx/sysmodule/`. for this, you need devkitpro and devkita64 installed, as well as libnx. then you can run `make` to build it. then, put the file called `sys-scope.nsp` onto the sd
card of your homebrewed switch at `/atmosphere/contents/420000000005C09E/exefs.nsp`. you can put the `toolbox.json` file right next to it in that directory as well, and then use something like
ovlSysmodule to start the module.

### running

once you've started the module on your switch, figure out its ip address (you can find this in the internet settings). when you start periscope on your computer,
you will be greeted with a configuration window. type the ip into the box that asks for it, and select your skin (which i hope you made), and then
click the button.
