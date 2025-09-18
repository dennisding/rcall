
use tokio;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::{TryRecvError};

pub struct Bichannel<T> {
    sender: mpsc::Sender<T>,
    receiver: mpsc::Receiver<T>
}

pub struct Sender<T> {
    sender: mpsc::Sender<T>
}

impl<T> Sender<T> {
    pub fn new(sender: mpsc::Sender<T>) -> Self {
        Sender {
            sender
        }
    }

    pub fn send(&self, value: T) {
        let _ = self.sender.try_send(value);
    }
}

impl<T> Bichannel<T> {
    pub fn build_channel(sender: mpsc::Sender<T>, receiver: mpsc::Receiver<T>) -> Bichannel<T> {
        Bichannel::<T> {
            sender,
            receiver
        }
    }

    pub fn new(size: usize) -> (Bichannel<T>, Bichannel<T>) {
        let (sender1, receiver1) = tokio::sync::mpsc::channel::<T>(size);
        let (sender2, receiver2) = tokio::sync::mpsc::channel::<T>(size);
        let channel1 = Self::build_channel(sender1, receiver2);
        let channel2 = Self::build_channel(sender2, receiver1);

        (channel1, channel2)
    }

    pub fn clone_sender(&self) -> Sender<T> {
        Sender::new(self.sender.clone())
    }

    pub fn send(&self, value: T) {
        // self.sender.try_send(value)
        if let Err(err) = self.sender.try_send(value) {
            println!("error in send: {}", err);
        }
    }

    pub fn try_recv(&mut self) -> Option<T> {
        if let Ok(msg) = self.receiver.try_recv() {
            return Some(msg);
        }
        
        None
    }

    pub async fn recv(&mut self) -> Option<T> {
        self.receiver.recv().await
    }
}