use std::{sync::mpsc::Receiver, thread, time::Duration};

pub struct Job {
    repeat: Option<Duration>,
    action: fn(),
}

pub struct Scheduler {
    reciever: Receiver<Message>,
    jobs: Vec<Job>,
}

pub enum Message {
    NewJob(Job),
}

impl Scheduler {
    fn run(&self) {
        thread::spawn(move || loop {
            self.reciever.recv();
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::Job;
    use std::time::Duration;

    use crate::Scheduler;

    #[test]
    fn it_works() {
        let mut scheduler = Scheduler { jobs: Vec::new() };
        scheduler.jobs.push(Job {
            repeat: Some(Duration::from_secs(5)),
            action: || println!("Hello World"),
        });
        scheduler.run();
    }
}
