#Car USB Accessory Mode
On USB-Device plugged in, try to set Accessory Mode, in case of success, a new usb device appears.

#Setup
 * checkout 
 * Install native dependencies:
   * Debian/Ubuntu: `sudo apt-get install libusb-1.0-0-dev`
 * build
   * `./build.release.sh`
   
# Background

For further information on `Android Open Accessory Protocol`, see https://source.android.com/devices/accessories/aoa2

# Inspect Debian package
`dpkg-deb -c target/debian/*.deb`

sudo apt-get install gcc-arm-linux-gnueabihf

# ~/.cargo/config

[target.armv7-unknown-linux-gnueabihf]
linker = "arm7-linux-gnueabihf-gcc"