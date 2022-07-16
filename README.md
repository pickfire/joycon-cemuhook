joycon-cemuhook
===============

Support for cemuhook's UDP protocol for joycond devices for use with emulators
like Dolphin, Cemu, Citra, Yuzu, etc.

Based on https://github.com/joaorb64/joycond-cemuhook, a lot of heavy lifting
is done in joycon and pad-motion crates, mostly just glue code here.

Work in progress, TODO
- controller initialization, still need to run `joycond-cemuhook.py` to let controller join a slot
- multiple controllers support, only one controller is supported now
- hotplugging, it only detects controller when application start
- requires root support, most likely on `joy` side
- print out some useful debugging information like device connect/disconnect

How to use
----------

- Install dkms-hid-nintendo (if your kernel doesn't include the `hid_nintendo` driver)
  - `hid_nintendo` did not work for me since it kernel panics https://github.com/DanielOgorchock/linux/issues/36
- Install the joycond userspace driver
- As for now, still need to run `joycond-cemuhook` to let controller join slot
- Clone this repository, `git clone https://github.com/pickfire/joycon-cemuhook`
- Navigate into the directory with `cd joycon-cemuhook/`
- Build with `cargo build --release`
- Run with `sudo target/release/joycon-cemuhook`

From now on, you'll only need to run `joycon-cemuhook` from a terminal on its directory.

- Connect your Nintendo Switch controllers and assign them as intended (using the respective L+R)
- Open a compatible emulator and enable cemuhook UDP motion input
