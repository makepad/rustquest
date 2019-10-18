use crate::App;
use jni::sys::{jobject, JNIEnv, JavaVM};
use libandroid_sys::ANativeWindow;
use std::ptr;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, TryRecvError};
use std::thread;
use std::thread::JoinHandle;

pub struct AppThread {
    env: *mut JNIEnv,
    activity: jobject,
    sender: Sender<Message>,
    thread: Option<JoinHandle<()>>,
    window: *mut ANativeWindow,
}

impl AppThread {
    pub fn new(env: *mut JNIEnv, activity: jobject) -> AppThread {
        let vm = unsafe {
            let mut vm: *mut JavaVM = ptr::null_mut();
            ((**env).GetJavaVM.unwrap())(env, &mut vm);
            vm
        };
        let activity = unsafe { ((**env).NewGlobalRef.unwrap())(env, activity) };
        let (sender, receiver) = mpsc::channel();
        sender.send(Message::OnCreate(vm, activity)).unwrap();
        AppThread {
            env,
            activity,
            sender,
            thread: Some(thread::spawn(move || {
                let mut app = None;
                logi!("entering event loop");
                loop {
                    match receiver.try_recv() {
                        Ok(message) => match message {
                            Message::OnCreate(vm, activity) => {
                                app = Some(App::new(vm, activity));
                            }
                            Message::OnStart => {}
                            Message::OnResume => {
                                app.as_mut().unwrap().set_resumed(true);
                            }
                            Message::OnPause => {
                                app.as_mut().unwrap().set_resumed(false);
                            }
                            Message::OnStop => {}
                            Message::OnDestroy => {
                                break;
                            }
                            Message::SurfaceCreated(window) => {
                                app.as_mut().unwrap().set_window(window);
                            }
                            Message::SurfaceDestroyed => {
                                app.as_mut().unwrap().set_window(ptr::null_mut());
                            }
                        },
                        Err(TryRecvError::Empty) => {},
                        Err(TryRecvError::Disconnected) => panic!(),
                    }

                    app.as_mut().unwrap().render_frame();
                }
                logi!("leaving event loop");
            })),
            window: ptr::null_mut(),
        }
    }

    pub fn on_start(&self) {
        self.sender.send(Message::OnStart).unwrap();
    }

    pub fn on_resume(&self) {
        self.sender.send(Message::OnResume).unwrap();
    }

    pub fn on_pause(&self) {
        self.sender.send(Message::OnPause).unwrap();
    }

    pub fn on_stop(&self) {
        self.sender.send(Message::OnStop).unwrap();
    }

    pub fn on_destroy(&self) {
        self.sender.send(Message::OnDestroy).unwrap();
    }

    pub fn surface_created(&mut self, env: *mut JNIEnv, surface: jobject) {
        let window = unsafe { libandroid_sys::ANativeWindow_fromSurface(env as _, surface as _) };
        self.window = window;
        self.sender.send(Message::SurfaceCreated(window)).unwrap();
    }

    pub fn surface_changed(&mut self, env: *mut JNIEnv, surface: jobject) {
        let window = unsafe { libandroid_sys::ANativeWindow_fromSurface(env as _, surface as _) };
        if window != self.window {
            if !self.window.is_null() {
                unsafe {
                    libandroid_sys::ANativeWindow_release(self.window);
                }
                self.window = ptr::null_mut();
                self.sender.send(Message::SurfaceDestroyed).unwrap();
            }

            if !window.is_null() {
                self.window = window;
                self.sender.send(Message::SurfaceCreated(window)).unwrap();
            }
        } else {
            if !window.is_null() {
                unsafe {
                    libandroid_sys::ANativeWindow_release(window);
                }
            }
        }
    }

    pub fn surface_destroyed(&mut self) {
        unsafe {
            libandroid_sys::ANativeWindow_release(self.window);
        }
        self.window = ptr::null_mut();
        self.sender.send(Message::SurfaceDestroyed).unwrap();
    }
}

impl Drop for AppThread {
    fn drop(&mut self) {
        self.thread.take().unwrap().join().unwrap();
        unsafe { ((**self.env).DeleteGlobalRef.unwrap())(self.env, self.activity) }
    }
}

enum Message {
    OnCreate(*mut JavaVM, jobject),
    OnStart,
    OnResume,
    OnPause,
    OnStop,
    OnDestroy,
    SurfaceCreated(*mut ANativeWindow),
    SurfaceDestroyed,
}

unsafe impl Send for Message {}
