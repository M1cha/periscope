# periscope
homebrew solution for nintendo switch input display

this is heavily inspired by openjoystickdisplay / OJDS-NX, which is implemented almost the same way. however, openjoystickdisplay has been abandonware for multiple years, and the OJDS-NX project was recently deleted.
therefore, i have reimplemented both sides of the equation, specifically for switch, and hopefully a little bit better.

### simple installation
you can just get prebuild stuff from the [releases page](https://codeberg.org/periwinkle/periscope/releases). if you want to build it yourself then read on.

### current features
* switch sysmodule that grabs inputs
* desktop viewer program that connects to the switch and pulls inputs from the sysmodule
* support for up to 8 players (pro controller, dual joy-con, or single joy-con (*i think*))
* custom skin support
* configuration dialog box
* tesla overlay that shows switch's IP address and allows configuration of which controllers to capture

### planned features
* a default skin to distribute with this that actually looks ok
* better desktop configuration
* support for gyro
* support for accelerometer (likely difficult)

### skin configuration
each skin lives in its own directory in the config folder (e.g. `~/.config/periscope/` on linux, `C:\Users\YourUser\AppData\Roaming\periwinkle\periscope\config` on windows). inside this directory
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
[player1]
buttons.a = {image = "button.png", pos = {x = 50, y = 50}}
buttons.b = {image = "button.png", pos = {x = 40, y = 60}}
# repeat for each button ...
ls = {image = "stick.png", pos = {x = 100, y = 100}, range = 50}
# repeat for right stick...

[player2]
# ...
```

if you leave a button or joystick unconfigured, it simply will not be rendered.

### building
if you have devkita64 and cargo installed properly, you should be able to simply run `make` in the root of this repository to build the sysmodule, overlay, and desktop viewer.

the viewer binary will be in desktop/target/release/periscope or desktop/target/debug/periscope depending on whether you build a release or debug executable.

the sysmodule will be at nx/sysmodule/sys-scope.nsp. put this file onto your switch SD card at `/atmosphere/contents/420000000005C09E/exefs.nsp` along with `toolbox.json`. you can then use something like ovlSysmodule
to start it.

the overlay binary will be at nx/overlay/periscope-overlay.ovl. put this in `/switch/.overlays`. you'll need tesla installed to run it, of course.

### running
once sys-scope is running on your switch, you can use the overlay or the wifi settings to determine your IP address.
when you start periscope on your computer, you will be greeted with a configuration window. type the ip into the box that asks for it, and select your skin (which i hope you made), and then
click the button.

### troubleshooting
usually, you can fix the problem by stopping and restarting the sysmodule. if not, feel free to open an issue.
