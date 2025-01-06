use crate::WorkerMessage::{End, Value};
use rand::Rng;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

enum WorkerMessage {
    End,
    Value(i32),
}

fn run_on_threads(threads: i32, jobs: i32) -> i32 {
    let (mut job_sender, job_receiver) = spmc::channel();
    let (result_sender, result_receiver) = mpsc::channel();

    let mut workers = Vec::new();
    for n in 0..threads {
        let receiver = job_receiver.clone();
        let sender = result_sender.clone();
        workers.push(thread::spawn(move || {
            let mut result = 0;
            let mut count = 0;
            loop {
                match receiver.recv().unwrap() {
                    End => {
                        sender.send((n, result, count)).unwrap();
                        break;
                    }
                    Value(v) => {
                        let duration = Duration::from_millis(rand::rng().random_range(10..20));
                        #[cfg(debug_assertions)]
                        println!(
                            "Worker {} received: {} and sleeping for {:?}",
                            n, v, duration
                        );
                        count += 1;
                        result += v;
                        thread::sleep(duration)
                    }
                }
            }
        }));
    }

    for i in 0..jobs {
        job_sender.send(Value(i)).unwrap();
    }
    for _ in 0..threads {
        job_sender.send(End).unwrap();
    }

    let mut result = 0;
    for _ in 0..threads {
        let msg = result_receiver.recv().unwrap();
        println!("Spawner received: {:?}", msg);
        result += msg.1;
    }

    for worker in workers {
        worker.join().unwrap();
    }

    result
}

fn main() {
    let n = 10000;
    let res = run_on_threads(50, n);
    println!("Sum number from 0..<{n} is {res}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_sum_numbers() {
        let n = 100;
        assert_eq!(n * (n - 1) / 2, run_on_threads(5, n));
    }

    #[test]
    fn should_sum_numbers_with_more_threads_than_numbers() {
        let n = 7;
        assert_eq!(n * (n - 1) / 2, run_on_threads(10, n));
    }
}
