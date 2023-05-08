use {
    futures::{
        future::{BoxFuture, FutureExt},
        task::{waker_ref, ArcWake},
    },

    std::{
        future::Future,
        sync::mpsc::{sync_channel, Receiver, SyncSender},
        sync::{Arc, Mutex},
        task::{Context, Poll},
        time::Duration,
    },

    timer_future::TimerFuture,
};


struct Task {
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    task_sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
    // 被唤醒后执行
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self
            .task_sender
            .send(cloned)
            .expect("task queue full");
    }
}

struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

impl Spawner {
    // fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
    fn spawn(self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });

        self.task_sender.send(task).expect("task queue full");
    }
}

struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

impl Executor {
    fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                let waker = waker_ref(&task);
                let cx = &mut Context::from_waker(&*waker);

                if future.as_mut().poll(cx).is_pending() {
                    *future_slot = Some(future);
                }
            }
        }
    }
}


fn new_executor_and_spawner() -> (Executor, Spawner) {
    const MAX_QUEUED_TASKS: usize = 10000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor{ ready_queue }, Spawner{ task_sender })
}


fn main() {
    let (exector, spawner) = new_executor_and_spawner();

    // 将这个 future 放入 exectur 的 Queue 中
    spawner.spawn(async {
        println!("task crated");
        // 实现了 Future 特征的 struct 使用 await: 一直等到 ready 才返回
        TimerFuture::new(Duration::new(2, 0)).await;
    });

    // drop(spawner);  // 为了让 SyncSender 关闭, 共两个, 关掉 spawn 的那个
    // 将 spawn 方法由 &self 改成了 self, 不需要显式的 drop 了
    exector.run();
}