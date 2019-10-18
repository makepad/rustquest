use jni::sys::{jobject, JNIEnv};
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::JoinHandle;

pub struct AppThread {
    env: *mut JNIEnv,
    activity: jobject,
    sender: Sender<Message>,
    thread: Option<JoinHandle<()>>,
}

impl AppThread {
    pub fn new(env: *mut JNIEnv, activity: jobject) -> AppThread {
        let activity = unsafe { ((**env).NewGlobalRef.unwrap())(env, activity) };
        let (sender, receiver) = mpsc::channel();
        let thread = Some(thread::spawn(move || {
            logi!("entering event loop");
            loop {
                match receiver.recv().unwrap() {
                    Message::OnStart => {}
                    Message::OnResume => {}
                    Message::OnPause => {}
                    Message::OnStop => {}
                    Message::OnDestroy => {
                        break;
                    }
                }
            }
            logi!("leaving event loop");
        }));
        AppThread {
            env,
            activity,
            sender,
            thread,
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

    pub fn surface_created(&mut self, _env: *mut JNIEnv, _surface: jobject) {
        // TODO
    }

    pub fn surface_changed(&mut self, _env: *mut JNIEnv, _surface: jobject) {
        // TODO
    }

    pub fn surface_destroyed(&mut self) {
        // TODO
    }
}

impl Drop for AppThread {
    fn drop(&mut self) {
        self.thread.take().unwrap().join().unwrap();
        unsafe { ((**self.env).DeleteGlobalRef.unwrap())(self.env, self.activity) }
    }
}

enum Message {
    OnStart,
    OnResume,
    OnPause,
    OnStop,
    OnDestroy,
}
