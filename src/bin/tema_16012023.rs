//Domanda 4: Una barriera è un costrutto di sincronizzazione usato per regolare l'avanzamento relativo della computazione di più thread. 
// All'atto della costruzione di questo oggetto, viene indicato il numero N di thread coinvolti. 

// Non è lecito creare una barriera che coinvolga meno di 2 thread. 

// La barriera offre un solo metodo, wait(), il cui scopo è bloccare temporaneamente l'esecuzione del thread che lo ha invocato, non ritornando fino a che non sono giunte 
// altre N-1 invocazioni dello stesso metodo da parte di altri thread: quando ciò succede, la barriera si sblocca e tutti tornano. Successive invocazioni del metodo wait() 
// hanno lo stesso comportamento: la barriera è ciclica.

// Attenzione a non mescolare le fasi di ingresso e di uscita!

// Una RankingBarrier è una versione particolare della barriera in cui il metodo wait() restituisce un intero che rappresenta l'ordine di arrivo: il primo thread ad avere 
// invocato wait() otterrà 1 come valore di ritorno, il secondo thread 2, e così via. All'inizio di un nuovo ciclo, il conteggio ripartirà da 1.

// Si implementi la struttura dati RankingBarrier a scelta nei linguaggi Rust o C++ '11 o successivi.
use std::thread::{ spawn };
use std::sync::{ Arc, Condvar, Mutex };

struct BarrierState {
    arrival: usize,
    waiting: usize
}

struct RankingBarrier {
    n: usize,
    condvar: Condvar,
    state: Mutex<BarrierState>
}

impl RankingBarrier {
    
    pub fn new(n: usize) -> RankingBarrier {
        let arrival = 0;
        let waiting = 0;
        let state = BarrierState {
            arrival,
            waiting
        };
        let condvar = Condvar::new();
        RankingBarrier {
            n,
            condvar,
            state: Mutex::new(state)
        }
    }

    pub fn wait(&self, i: usize) -> usize { 
        let mut lock = self.state.lock().unwrap();
        let local_arrival = lock.arrival;
        lock.arrival += 1;
        lock.waiting += 1;
        let _guard = self.condvar.wait_while(lock, |state| state.waiting  < self.n - 1).unwrap();
        self.condvar.notify_all();
        local_arrival
    }

}

pub fn main() {
   let n = 5;
   let mut handles = vec![];
   let ranking_barrier = Arc::new(RankingBarrier::new(n));
   
   for i in 0..n {
       let mut ranking_barrier_clone: Arc<RankingBarrier> = Arc::clone(&ranking_barrier);
       let handle = spawn(move || {
           println!("Waiting for barrier to open from thread {}",i);
           let arrival = ranking_barrier_clone.wait(i);
           println!("Thread {} arrived at {}", i, arrival);
        });
        handles.push(handle);
   }

   for handle in handles {
       let _ = handle.join();
   }

}