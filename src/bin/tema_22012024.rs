/*
La struttura MultiChannel implementa il concetto di canale con molti mittenti e molti ricevitori.
I messaggi inviati a questo tipo di canale sono composti da singoli byte che vengono recapitati  a
tutti i ricevitori attualmente collegati.
Metodi:
      new() -> Self // crea un nuovo canale senza alcun ricevitore collegato
      subscribe(&self) -> mpsc::Receiver<u8> // collega un nuovo ricevitore al canale: da quando
                                             // questo metodo viene invocato, gli eventuali byte
                                             // inviati al canale saranno recapitati al ricevitore.
                                             // Se il ricevitore viene eliminato, il canale
                                             // continuerà a funzionare inviando i propri dati
                                             // ai ricevitori restanti (se presenti), altrimenti
                                             // ritornerà un errore
      send(&self, data: u8) -> Result<(), SendError<u8>>
                                            // invia a tutti i sottoscrittori un byte
                                            // se non c'è alcun sottoscrittore, notifica l'errore
*/

use std::{sync::{mpsc::{channel, Receiver, SendError, Sender}, Arc, RwLock}, thread::{sleep, spawn}, time::Duration};

use rand::{thread_rng, Rng};

struct MultiChannel {
    txs: RwLock<Vec<Sender<u8>>>
}

impl MultiChannel {

    pub fn new() -> MultiChannel {
        MultiChannel {
            txs: RwLock::new(Vec::new())
        }
    }

    pub fn subscribe(&self) -> Receiver<u8> {
        let ( tx, rx ) = channel();
        self.txs.write().unwrap().push(tx);
        rx
    }

    pub fn send(&self, data: u8) -> Result<(), SendError<u8>>{
        let mut txs = self.txs.write().unwrap();

        if txs.len() == 0 {
            return Err(SendError(0));
        }

        let mut i = 0;
        for tx in txs.clone() {
            let res = tx.send(data);
            if res.is_err() {
                txs.remove(i);
            }
            i+=1;
        }

        return Ok(());
    }
}

pub fn main() {
    let multi_channel = Arc::new(MultiChannel::new());
    let mut handles = vec![];
    let mut rng = thread_rng();
    let random_sender = rng.gen_range(0..10);
    println!("Sender {}: I'm going to send my id to all the receivers through a multi channel", random_sender);

    for i in 0..10 {
        let multi_channel_clone = Arc::clone(&multi_channel);
        let handle = spawn(move || {
            let rx = multi_channel_clone.subscribe();
            println!("Receiver {}: From now on i'm ready to receive messages",i);
            let mut data = rx.recv();
            while data.is_err() {
                data = rx.recv();
            }
            println!("Receiver {}: Received a message from {}",i,data.ok().unwrap());
        });
        handles.push(handle);
    }

    let handle = spawn(move || {
        sleep(Duration::from_secs(10));
        let _ = multi_channel.send(random_sender);
        println!("Sender {}: I sent a message to all the receivers", random_sender);
    });

    for (i,handle) in handles.into_iter().enumerate() {
        let _ = handle.join();
        println!("Closed connection with Receiver {}",i);
    } 
    let _ = handle.join();
}