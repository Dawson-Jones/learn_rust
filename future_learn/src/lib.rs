use std::pin::Pin;
use std::future::Future;
use std::task::{Context, Poll};
use futures::task::{self, ArcWake};
use crossbeam::channel;
use std::sync::{Arc, Mutex};



pub struct Task {
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
    executor: channel::Sender<Arc<Task>>,
}

impl Task {
    pub fn scheduled(self: &Arc<Self>) {
        self.executor.send(self.clone());
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.scheduled();
    }
}

impl Task {
    pub fn poll(self: Arc<Self>) {
        let waker = task::waker(self.clone());
        let mut cx = Context::from_waker(&waker);
        let mut future = self.future.try_lock().unwrap();

        let _ = future.as_mut().poll(&mut cx);
    }

    pub fn spawn<F>(future: F, sender: &channel::Sender<Arc<Task>>)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = Arc::new(Task {
            future: Mutex::new(Box::pin(future)),
            executor: sender.clone(),
        });

        let _ = sender.send(task);
    }
}

pub struct MiniTokio {
    scheduled: channel::Receiver<Arc<Task>>,
    sender: channel::Sender<Arc<Task>>,
}

impl MiniTokio {
    pub fn new() -> Self {
        let (sender, scheduled) = channel::unbounded();
        MiniTokio { sender, scheduled }
    }

    pub fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + Send + 'static
    {
        // self.tasks.push_back(Box::pin(future));
        Task::spawn(future, &self.sender);
    }

    pub fn run(&mut self) {
        while let Ok(task) = self.scheduled.recv() {
            task.poll();
        }
    }
}