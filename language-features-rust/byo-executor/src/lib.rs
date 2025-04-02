use std::{
    collections::VecDeque,
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Wake, Waker},
    time::{Duration, Instant},
};

#[derive(Default)]
pub struct Runtime {
    queue: Queue,
}

impl Runtime {
    pub fn spawn<F>(&self, fut: F)
    where
        F: Future<Output = ()> + Send + Sync + 'static,
    {
        Task::spawn(fut, self.queue.clone());
    }

    pub fn run(&self) {
        loop {
            let task = {
                let mut queue = self.queue.lock().unwrap();
                match queue.pop_front() {
                    Some(task) => task,
                    None => {
                        if queue.is_empty() {
                            break;
                        } else {
                            continue;
                        }
                    }
                }
            };
            let mut fut = task.fut.lock().unwrap();
            let waker = task.waker();
            if fut
                .as_mut()
                .poll(&mut Context::from_waker(&waker))
                .is_pending()
            {
                waker.wake();
            }
        }
    }
}

type Queue = Arc<Mutex<VecDeque<Arc<Task>>>>;

struct Task {
    fut: Mutex<Pin<Box<dyn Future<Output = ()> + Send + Sync>>>,
    queue: Queue,
}

impl Task {
    fn spawn<F>(fut: F, queue: Queue)
    where
        F: Future<Output = ()> + Send + Sync + 'static,
    {
        let task = Arc::new(Self {
            fut: Mutex::new(Box::pin(fut)),
            queue,
        });
        task.queue.lock().unwrap().push_back(task.clone());
    }

    fn waker(self: &Arc<Self>) -> Waker {
        self.clone().into()
    }
}

impl Wake for Task {
    fn wake(self: Arc<Self>) {
        self.queue.lock().unwrap().push_back(self.clone());
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.queue.lock().unwrap().push_back(self.clone());
    }
}

pub fn sleep(duration: Duration) -> impl Future<Output = ()> + Send + Sync + 'static {
    Sleep::new(duration)
}

struct Sleep {
    start: Instant,
    duration: Duration,
}

impl Sleep {
    fn new(duration: Duration) -> Self {
        Self {
            start: Instant::now(),
            duration,
        }
    }
}

impl Future for Sleep {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() - self.start >= self.duration {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}
