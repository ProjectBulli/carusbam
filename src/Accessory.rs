// @see https://source.android.com/devices/accessories/aoa2
/*
use US

enum AOA {
    v1,
    v2
}

struct USBTypes {
    version: AOA,
    productId: Product
}
    AOAv1    0x2D00    accessory    Provides two bulk endpoints for communicating with an Android application.
0x2D01    accessory + adb    For debugging purposes during accessory development. Available only if the user has enabled USB Debugging in the Android device settings.
AOAv2    0x2D02    audio    For streaming audio from an Android device to an accessory.
0x2D03    audio + adb
0x2D04    accessory + audio
0x2D05    accessory + audio + adb

*/
