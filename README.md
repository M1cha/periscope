# periscope
homebrew solution for nintendo switch input display

## simple installation
you can just get prebuilt stuff from the [releases page](https://codeberg.org/periwinkle/periscope/releases). if you want to build it yourself then read on.

## troubleshooting
usually, you can fix the problem by restarting the sysmodule (which you can do from the overlay). if not, feel free to open an issue, [contact me on matrix](https://matrix.to/#/@periwinkle:periwinkle.sh) or [ask in #periscope in my discord](https://discord.gg/XCyNdTWznJ).

### current features
* switch sysmodule that grabs inputs
* desktop viewer program that connects to the switch and pulls inputs from the sysmodule
* support for up to 8 players (pro controller, dual joy-con, or single joy-con (*i think*))
* custom skin support
* configuration dialog box
* tesla overlay that shows switch's IP address and allows configuration of which controllers to capture

### planned features
* better desktop configuration
* support for gyro
* support for accelerometer (likely difficult)

### building
To build this, you will need devkitA64 installed ([instructions here](https://devkitpro.org/wiki/Getting_Started)) as well as rust and cargo (either from your system package manager or follow [instructions here](https://rustup.rs)).


if you have devkitA64 and cargo installed properly, you should be able to simply run `make` in the root of this repository to build the sysmodule, overlay, and desktop viewer.

the viewer binary will be in desktop/target/release/periscope or desktop/target/debug/periscope depending on whether you build a release or debug executable.

the overlay binary will be at nx/overlay/periscope-overlay.ovl. put this in `/switch/.overlays/` on your switch SD card. you'll need [tesla](https://github.com/werwolv/tesla-menu) and [ovlloader](https://github.com/WerWolv/nx-ovlloader) installed for
it. to open tesla, the default keybinding is L + Dpad Down + RS. from there you can select the option labeled periscope to open periscope's overlay.

the sysmodule will be at nx/sysmodule/sys-scope.nsp. put this file onto your switch SD card at `/atmosphere/contents/420000000005C09E/exefs.nsp` and put `toolbox.json` in the same folder. you can start it with [ovlSysmodules](https://github.com/WerWolv/ovl-sysmodules) (another Tesla overlay)
or from inside the periscope overlay itself, which has options to start and restart sys-scope.

### running
once you have everything installed on your switch, you can open tesla with L + Dpad down + RS, and then open the periscope overlay. this can be used to start the sysmodule (if it's not already running)
as well as to get the IP address of your switch. You can also start the sysmodule with other tools, and figure out your IP from the wifi settings on the switch.

you can download a skin zip file from the [releases](https://codeberg.org/periwinkle/periscope/releases) page, and place it into the configuration directory mentioned in the release page for your platform.
when you start periscope on your computer, you will be greeted with a configuration window. type the ip into the box that asks for it, and select your skin, and theh
click the button to connect to your switch and start capturing inputs.

### skin configuration
each skin lives in its own directory in the config folder (e.g. `~/.config/periscope/` on linux,
`C:\Users\YourUser\AppData\Roaming\periwinkle\periscope\config` on windows, `/Users/YourUser/Library/Application Support/periwinkle.periscope` on mac). inside this directory
is a `skin.toml` file and every image you need for the skin. you need at least a background image and one other, to use for buttons and joysticks.
each button and joystick can be a different image, if you want. each button/stick has a `pos`, which is where on the screen it will be displayed. the position refers
to the top left corner of the image. x increases to the right and y towards the bottom, starting at the top left of the window. joysticks also have a `range`, which is the radius of the circle that the joystick
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

if you leave a button, joystick, or player unconfigured, it simply will not be rendered.

