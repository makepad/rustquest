#!/bin/bash
PATH=$ANDROID_HOME/build-tools/28.0.3:$PATH
PATH=$NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin:$PATH

pushd native >/dev/null
    cargo build
popd >/dev/null
if [ -d build ]; then
    rm -rf build
fi
mkdir build
pushd build >/dev/null
    javac\
        -classpath $ANDROID_HOME/platforms/android-26/android.jar\
	-d .\
        $(find ../android/src -name "*.java")
    dx --dex --output classes.dex $(find . -name "*.class")
    mkdir -p lib/arm64-v8a
    pushd lib/arm64-v8a >/dev/null
        cp ../../../native/target/aarch64-linux-android/debug/libnative.so .
        cp $OVR_HOME/VrApi/Libs/Android/arm64-v8a/Debug/libvrapi.so .
    popd >/dev/null
    aapt\
        package\
        -F rustquest.apk\
        -I $ANDROID_HOME/platforms/android-26/android.jar\
        -M ../android/AndroidManifest.xml\
        -f
    aapt add rustquest.apk classes.dex > /dev/null
    aapt add rustquest.apk lib/arm64-v8a/libnative.so
    aapt add rustquest.apk lib/arm64-v8a/libvrapi.so
    apksigner\
	    sign\
	    -ks ../debug.keystore\
	    --ks-key-alias androiddebugkey\
	    --ks-pass pass:android\
	    rustquest.apk
popd >/dev/null

