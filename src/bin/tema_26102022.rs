// Domanda 1: Si illustrino le differenze tra stack e heap. Insieme alle differenze indicare per i
// seguenti costrutti Rust, in modo dettagliato, dove si trovano i dati che li compongono: Box<[T]>, RefCell<T>, &[T].
//
// Stack e heap sono due address space logicamente separati dall'esistenza del sistema operativo ma
// fisicamente situati nella ram, la porzione di memoria chiamata stack ha il compito di tenere in
// memoria dati fino a che il codice che si riferisce ad essi è in esecuzione mentre lo heap
// (tradotto come catasta) è una porzione di memoria dedicata all'allocazione di dati senza
// obblighi sulla deallocazione, è responsabilità del programmatore deallocare opportunamente.
// Entrambe le zone della memoria vengono create dal OS alla creazione di un thread e sono quindi
// non sovrascrivibili da thread esterni ad esso. Quando l'intero thread termina esse vengono
// tuttavia deallocate ma non sovrascritte.
// 
// In Rust una struttura dati del tipo Box<[T]> è allocata nella maniera seguente:
// [T] ammesso che T non sia un dato puntato viene allocato sullo stack in maniera contigua per
// tutta la sua lunghezza dato che [] ha una dimensione conosciuta in fase di compilazione.
// Box<[T]> invece dato [T] allocato sullo stack viene allocato sullo heap contenendo una copia del
// sopracitato [T].
//
// RefCell<T> è allocata sullo stack contenendo due valori, il primo indica se il dato è stato
// prestato (esiste almeno un riferimento esterno ad esso), il secondo è una copia effettiva del
// dato.
//
// &[T] è allocato sullo stack, come già illustrato [T] è scritto completamente sullo stack mentre
// &[T] è scritto anche sullo stack e contiene solo l'indirizzo di memoria del primo byte di [T]
//
// Domanda 2: Un sistema concorrente può essere implementato creando più thread nello stesso processo, creando più
// processi basati su un singolo thread o basati su più thread. Si mettano a confronto i corrispettivi modelli di
// esecuzione e si evidenzino pregi e difetti in termini di robustezza, prestazioni, scalabilità e semplicità di
// sviluppo di ciascuno di essi
//
// Entrambi gli approcci descritti prevedono l'esecuzione del codice in parallelo, il primo, la
// programmazione concorrente prevede un interazione minima tra i thread che vengono inseguiti in
// parallelo mentre il secondo chiamato programmazione asincrona è più indicato per l'esecuzione di
// porzioni di codice in parallelo che hanno dipendenze di dato da operazioni bloccanti comuni.
// In termini di robustezza del codice gli approcci descritti si basano entrambi su integrazione
// con il sistema operativo e bindings ad esso fornite dal linguaggio in cui il parallelismo viene
// implementato, la programmazione parallela in rust viene definita fearless o senza paure in
// quanto gli errori sono limitati in gran parte alla fase di compilazione di un programma. La
// robustezza maggiore si potrebbe attribuire al modo in cui il programmatore implementa i due
// approcci, l'approccio asincrono essendo più legato al sistema operativo e al risultato
// delle operazioni di I/O può essere definito meno robusto. In termini di prestazioni l'approccio
// asincrono in quanto utilizza i processi richiede più tempo nella sostituzione del contesto in
// cui il codice deve essere eseguito mentre l'approccio concorrente ha dei dati comuni tra i
// thread richiede meno tempo per la sostituzione del contesto.
// In termini di scalabilità entrambi i metodi possono scalare bene a grandi numeri di esecutori
// paralleli seppure sono sempre limitati dal numero di core della cpu.
// In termini di semplicità di sviluppo l'approccio asincrono in rust ha un api che è molto legata
// al sistema operativo e nella scrittura richiede uno sforzo minore dovuto alla semplice
// conoscenza di quali sono le operazioni da svolgere in parallelo, quali quelle bloccanti e come
// far convergere i risultati di un operazione da un processo ad un altro.
//
// Domanda 3: In riferimento a programmi multi-thread, si indichi se la natura della loro esecuzione sia determinisitca o
// meno a priori. Si produca un esempio che dimostri tale affermazione
//
// I programmi multi threaded hanno comportamento deterministico se vengono implementati meccanismi
// di sincronizzazione se invece l'esecuzione del codice viene delegata all'ordine in cui i thread
// raggiungono le istruzioni non si può prevedere quale output verrà prodotto perchè la velocità
// dei singoli thread non dipende dal modo in cui viene scritto il programma ma dallo stato della
// cpu e da come il programma viene "distribuito" sui vari core del dispositivo dal sistema
// operativo, in questo caso senza sincronizzazione l'esecuzione è non-deterministica.
//
// Domanda 4: Un componente con funzionalità di cache permette di ottimizzare il comportamento di un sistema
// riducendo il numero di volte in cui una funzione è invocata, tenendo traccia dei risultati da essa restituiti a
// fronte di un particolare dato in ingresso.
// Per generalità, si assuma che la funzione accetti un dato di tipo generico K e restituisca un valore di tipo
// generico V.
// Il componente offre un unico metodo get(...) che prende in ingresso due parametri, il valore k (di tipo K,
// clonabile) del parametro e la funzione f (di tipo K -> V) responsabile della sua trasformazione, e
// restituisce uno smart pointer clonabile al relativo valore.
// Se, per una determinata chiave k, non è ancora stato calcolato il valore corrispondente, la funzione viene
// invocata e ne viene restituito il risultato; altrimenti viene restituito il risultato già trovato.
// Il componente cache deve essere thread-safe perché due o più thread possono richiedere
// contemporaneamente il valore di una data chiave: quando questo avviene e il dato non è ancora
// presente, la chiamata alla funzione dovrà essere eseguita nel contesto di UN SOLO thread, mentre gli
// altri dovranno aspettare il risultato in corso di elaborazione, SENZA CONSUMARE cicli macchina.
//
// Si implementi tale componente a scelta nei linguaggi C++ o Rust:
use std::collections::HashMap;
use std::fmt::Display;
use std::sync::{Arc, Condvar, Mutex};
use std::hash::Hash;

pub struct ParallelCache<K, V> 
where K: Eq + Hash + Clone, V: Display {
    map: Mutex<(HashMap<(K, fn(K) -> V), Arc<V>>, bool)>,
    condvar: Condvar,
}

impl<K, V> ParallelCache<K, V> 
where K: Eq + Hash + Clone, V: Display {
    pub fn new() -> ParallelCache<K, V> {
        ParallelCache {
            map: Mutex::new((HashMap::new(), false)),
            condvar: Condvar::new(),
        }
    }

    pub fn get(&self, input: K, function: fn(K) -> V) -> Arc<V> {

        let mut guard = self.condvar.wait_while(self.map.lock().unwrap(), |map| {
            !map.0.contains_key(&(input.clone(), function)) && map.1
        }).unwrap();

        let contains_key = guard.0.contains_key(&(input.clone(), function));
        if !contains_key {
            guard.1 = true;
            guard.0.insert((input.clone(), function), Arc::new(function(input.clone())));
            guard.1 = false;
            self.condvar.notify_all();
        } else {
            println!("Found already in cache: {}", guard.0.get(&(input.clone(), function)).unwrap());
        }

        return Arc::clone(guard.0.get(&(input, function)).unwrap());
    }
}

pub fn main() {}

#[cfg(test)]
mod test {
    use crate::ParallelCache;
    use std::{thread::spawn, sync::Arc};

    #[test]
    fn single_threaded() {
        let parallel_cache = ParallelCache::new();
        let function = |input| {
            input + 1
        };
        let result1 = parallel_cache.get(4,function);
        let result2 = parallel_cache.get(4,function);
        assert_eq!(result1, result2);
    }

    #[test]
    fn multi_threaded() {
        let parallel_cache = Arc::new(ParallelCache::new());
        let mut handles = vec![];
        let function = |input| {
            input + 1
        };

        for _ in 0..5 {
            let cache_clone = Arc::clone(&parallel_cache);
            let handle = spawn(move || {
                let _ = cache_clone.get(4,function);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}