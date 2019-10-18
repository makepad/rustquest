package com.makepad.rustquest;

import android.app.Activity;
import android.os.Bundle;
import android.util.Log;
import android.view.SurfaceHolder;
import android.view.SurfaceView;

public class MainActivity extends Activity implements SurfaceHolder.Callback {
    static {
        System.loadLibrary("native");
    }

    private static final String TAG = "rustquest";

    @Override
    public void onCreate(Bundle savedInstanceState) {
        Log.v(TAG, "MainActivity::onCreate()");
        super.onCreate(savedInstanceState);
        mSurfaceView = new SurfaceView(this);
        mSurfaceView.getHolder().addCallback(this);
        setContentView(mSurfaceView);
        mAppThread = JNI.onCreate(this);
    }

    @Override
    public void onStart() {
        Log.v(TAG, "MainActivity::onStart()");
        super.onStart();
        JNI.onStart(mAppThread);
    }

    @Override
    public void onResume() {
        Log.v(TAG, "MainActivity::onResume()");
        super.onResume();
        JNI.onResume(mAppThread);
    }

    @Override
    public void onPause() {
        Log.v(TAG, "MainActivity::onPause()");
        super.onPause();
        JNI.onPause(mAppThread);
    }

    @Override
    public void onStop() {
        Log.v(TAG, "MainActivity::onStop()");
        super.onStop();
        JNI.onStop(mAppThread);
    }

    @Override
    public void onDestroy() {
        Log.v(TAG, "MainActivity::onDestroy()");
        JNI.onDestroy(mAppThread);
        super.onDestroy();
        mAppThread = 0;
    }

    @Override
    public void surfaceCreated(SurfaceHolder holder) {
        Log.v(TAG, "MainActivity::surfaceCreated()");
        if (mAppThread != 0) {
            JNI.surfaceCreated(mAppThread, holder.getSurface());
        }
    }

    @Override
    public void surfaceChanged(SurfaceHolder holder, int format, int width, int height) {
        Log.v(TAG, "MainActivity::surfaceChanged()");
        if (mAppThread != 0) {
            JNI.surfaceChanged(mAppThread, holder.getSurface());
        }
    }

    @Override
    public void surfaceDestroyed(SurfaceHolder holder) {
        Log.v(TAG, "MainActivity::surfaceDestroyed()");
        if (mAppThread != 0) {
            JNI.surfaceDestroyed(mAppThread);
        }
    }

    private SurfaceView mSurfaceView;
    private long mAppThread;
}
