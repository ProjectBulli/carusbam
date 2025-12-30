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

# Android Open Accessory 2.0
see https://source.android.com/devices/accessories/aoa2

AOAv1    0x2D00    accessory    Provides two bulk endpoints for communicating with an Android application.
0x2D01    accessory + adb    For debugging purposes during accessory development. Available only if the user has enabled USB Debugging in the Android device settings.
AOAv2    0x2D02    audio    For streaming audio from an Android device to an accessory.
0x2D03    audio + adb
0x2D04    accessory + audio
0x2D05    accessory + audio + adb
