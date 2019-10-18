package com.makepad.rustquest;

import android.app.Activity;
import android.view.Surface;

public class JNI {
    public static native long onCreate(Activity activity);
    public static native void onStart(long app_thread);
    public static native void onResume(long app_thread);
    public static native void onPause(long app_thread);
    public static native void onStop(long app_thread);
    public static native void onDestroy(long app_thread);
    public static native void surfaceCreated(long app_thread, Surface surface);
    public static native void surfaceChanged(long app_thread, Surface surface);
    public static native void surfaceDestroyed(long app_thread);
}
