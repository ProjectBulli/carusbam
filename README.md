#Car USB Accessory Mode
On USB-Device plugged in, try to set Accessory Mode, in case of success, a new usb device appears-

https://github.com/anod/headunit/blob/master/app/src/main/java/info/anodsplace/headunit/connection/UsbAccessoryMode.kt 
https://github.com/mikereidis/headunit/blob/master/jni/hu_ush.c

sudo udevadm control --reload-rules

#Setup
 * checkout from ...
 * Install native dependencies:
   * Debian/Ubuntu: `sudo apt-get install libusb-1.0-0-dev`
   
 * build
   * `./build.release.sh`
