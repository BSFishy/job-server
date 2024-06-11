use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use rand::Rng;

struct Job {
    name: String,
    action: Box<dyn FnOnce(&JobServer) + Send>,
}

impl Job {
    fn new(name: String, action: impl FnOnce(&JobServer) + Send + 'static) -> Self {
        Job {
            name,
            action: Box::new(action),
        }
    }
}

#[derive(Clone)]
struct JobServer {
    job_count: Arc<AtomicUsize>,
    sender: async_channel::Sender<Job>,
}

impl JobServer {
    fn enqueue(&self, job: Job) {
        let count = self.job_count.fetch_add(1, Ordering::SeqCst);

        println!("job count: {}", count + 1);

        if let Err(err) = self.sender.send_blocking(job) {
            println!("send error: {}", err);

            panic!("unable to send");
        }
    }
}

fn main() {
    let count = thread::available_parallelism().unwrap().get();
    println!("Available parallelism: {}", count);

    let (tx, rx) = async_channel::unbounded::<Job>();

    let server = JobServer {
        job_count: Arc::new(AtomicUsize::new(0)),
        sender: tx,
    };

    // TODO: this should probably be thrown into the job server
    let mut threads = Vec::with_capacity(count);
    for i in 0..count {
        let rx = rx.clone();
        let server = server.clone();

        let builder = thread::Builder::new().name(format!("worker_{}", i));

        threads.push(
            builder
                .spawn(move || {
                    while !rx.is_closed() {
                        let job = match rx.recv_blocking() {
                            Ok(val) => val,
                            Err(_) => {
                                println!("Closing because empty");

                                // channel is empty and closed
                                break;
                            }
                        };

                        println!("Executing \"{}\" on worker {}", job.name, i);

                        (job.action)(&server);

                        if server.job_count.fetch_sub(1, Ordering::SeqCst) == 1 {
                            println!("closed because empty");
                            rx.close();
                        }
                    }
                })
                .unwrap(),
        );
    }

    server.enqueue(Job::new("Job 1".into(), |server| {
        println!("Starting job 1");

        for _ in 0..5 {
            server.enqueue(Job::new("work".into(), do_work));
        }

        println!("Finished job 1");
    }));

    println!("Joining");

    // TODO: this should probably be thrown into the job server
    for handler in threads {
        handler.join().unwrap();
    }

    println!("Finished");
}

fn do_work(server: &JobServer) {
    println!("Doing an arbitrary amount of work");
    thread::sleep(Duration::from_secs(rand::thread_rng().gen_range(1..=5)));

    if rand::random() {
        println!("Queuing more work");

        server.enqueue(Job::new("more work".into(), do_work));
    }
}
