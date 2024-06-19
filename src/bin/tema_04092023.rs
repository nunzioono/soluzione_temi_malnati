// Domanda 1: Attraverso un esempio pratico si illustri l’utilizzo dei Mutex nel linguaggio Rust. Qual è il
// meccanismo che consente il loro utilizzo in un contesto thread-safe?
//
use std::{ thread::spawn, sync::{ Arc, Mutex } };

fn mutex() {
    let count = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let count_clone = Arc::clone(&count);
        let handle = spawn(move || {
            let mut count = count_clone.lock().unwrap();
            println!("Count: {}", *count);
            *count += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

// L'utilizzo dei Mutex corrisponde all'utilizzo di un lucchetto su di un dato.
// Questo prevede che una volta aperto il lucchetto il dato venga acquisito da chi lo apre e
// venga impedito ad altri tentativi di aprire il lucchetto fino a quando l'attuale utilizzatore non lo chiude.
// Nei contesti thread-safe questo significa che non si possono verificare corse critiche o deadlock nel codice relativi al dato protetto dal Mutex.
// Ciò è garantito dal fatto che qualsiasi operazione di lettura avviene dopo il termine di un eventuale scrittura e non corrisponde mai ad un accesso mentre il dato viene scritto.
// Allo stesso tempo le scritture tra loro vengono ordinate nel ordine di apertura dei lucchetti.
//
// ----------------------------------------------------------------------------------------------------------------
//
//
// Domanda 2: Si descriva il concetto di "ownership" in Rust e come contribuisca a prevenire errori come le race
// condition e i dangling pointer rispetto ad altri linguaggi come C++
// 
// Con il concetto di ownership in Rust si definisce l'acquisizione di una variabile da parte di uno scopo.
// Questo concetto serve a stabilire per ogni porzione di memoria allocata (variabile) chi deve effettuarne la deallocazione al termine delle operazioni.
// Il compilatore rustc si preoccupa di deallocare le variabili in possesso di uno scopo quando esso termina. Allo stesso modo le variabili contenenti 
// puntatori o riferimenti ad altri dati sono deallocati quando lo scopo in cui sono allocati finisce. Inoltre un modulo del compilatore rustc chiamato borrow checker
// si occupa di controllare e segnalare errori di compilazione in caso si violino una o piu' delle seguenti regole:
// 
// 1. Ogni variabile può essere posseduta da un solo scopo.
// 2. Per ogni variabile possono esistere molteplici riferimenti non mutabili se non esistono riferimenti mutabili ad essa.
// 3. Per ogni variabile può esistere un solo riferimento mutabile se non vi sono riferimenti immutabili.
// 
// In questo modo rust tramite il compilatore garantisce che l'assenza di dangling pointers. Un dangling pointer è un puntatore ad un dato che non è allocato in memoria.
// 
// Per quanto riguarda le race conditions si tratta di un fenomeno che si verifica quando due o più thread cercano di eseguire una porzione di codice chiamata zona critica,
// nella quale si richiede esista un solo esecutore alla volta per non determinare comportamenti non-deterministici.
// In rust il concetto di possesso di una variabile risulta "comodo" anche nell'ambito del multi-threading perchè viene vietata l'acquisizione contemporanea delle variabili limitando le possibilità di race conditions.
// 
// ----------------------------------------------------------------------------------------------------------------
//
//
// Domanda 3: Si dimostri come sia possibile implementare il polimorfismo attraverso i tratti. Si fornisca anche un
// esempio concreto che faccia riferimento ad almeno due strutture diverse.
//
// Nell'implementazione del linguaggio Rust non è stato provvisto un reale supporto per la programmazione OOP, tuttavia 
// tramite costrutti innovativi chiamati Tratti che sono simili al concetto di Interfaccia in altri linguaggi è possibile
// implementare meccanismi di ereditarietà. Un tratto definisce i prototipi di metodi che una struttura dati che lo implementa dovrà implementare.
// Con polimorfismo si intende la possibilità che un dato possieda metodi derivanti da altri tipi di dato.

fn polymorphism() {
    
    trait Quad {
        fn sides(&self) -> usize {
            4
        }
    }

    struct Square {}

    impl Quad for Square {
        fn sides(&self) -> usize {
            4
        }
    }

    struct Rectangle {}

    impl Quad for Rectangle {
        fn sides(&self) -> usize {
            4
        }
    }

    let square = Square {};
    let rectangle = Rectangle {};
    assert_eq!(square.sides(), rectangle.sides());
    println!("Polymorphism works!");

}

// Domanda 4: Una cache è una struttura dati, generica, thread safe che consente di memorizzare coppie
// chiave/valore per un periodo non superiore ad una durata stabilita per ciascuna coppia.
// Nell'intervallo di validità associato alla coppia, richeste di lettura basate sulla chiave restituiscono il
// valore corrispondente, se presente. Trascorso tale periodo, eventuali richieste relative alla stessa
// chiave non restituiscono più il valore.
// Poiché il numero di richieste in lettura può essere molto maggiore delle richieste in scrittura, è
// necessario fare in modo che le prime possano sovrapporsi temporalmente tra loro, mentre le
// seconde dovranno necessriamente essere con accesso esclusivo. Per evitare la saturazione della
// struttura, quando si eseguno operazioni di scrittura, si provveda ad eseguire un ciclo di pulizia,
// eliminando le eventuali coppie scadute.
// Si implementi in Rust la struct  Cache<K: Eq+Hash, V> dotata dei seguenti metodi:
// pub fn new() -> Self // Crea una nuova istanza
// pub fn size(&self) -> usize // Restituisce il numero di coppie presenti nella mappa
// pub fn put(&self, k: K, v: V, d: Duration) -> () // Inserisce la coppia k/v con durata pari a d
// pub fn renew(&self, k: &K, d: Duration) -> Bool  // Rinnova la durata dell'elemento
// rappresentato dalla chiave k; restituisce true se la chiave esiste e non è scaduta, altrimenti
// restituisce false
// pub fn get(&self, k: &K) -> Option<Arc<V>> // Restituisce None se la chiave k è scaduta o non
// è presente nella cache; altrimenti restituisce Some(a), dove a è di tipo Arc<V>
// Si ricordi che Duration è una struttura contenuta in std::time, che rappresenta una durata non
// negativa. Può essere sommato ad un valore di tipo std::time::Instant (che rappresenta un
// momento specifico nel tempo) per dare origine ad un nuovo Instant, collocato più avanti nel
// tempo.

use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::hash::Hash;

struct Cache<K: Eq + Hash, V> {
    map: Mutex<HashMap<K, (Instant, Arc<V>)>>,
}

impl<K: Eq + Hash, V> Cache<K,V> {
    
    pub fn new() -> Cache<K,V> {
        Cache {
            map: Mutex::new(HashMap::new())
        }
    }

    pub fn size(&self) -> usize {
        self.map.lock().unwrap().len()
    }

    pub fn put(&mut self, k: K, v: V, d: Duration) {
        let mut map = self.map.lock().unwrap();
        let duetime = Instant::now() + d;
        let v_rc = Arc::new(v);

        map.entry(k)
        .and_modify(|value| {
            *value = (duetime, Arc::clone(&v_rc));
        })
        .or_insert((duetime, Arc::clone(&v_rc)));
    }

    pub fn renew(&self, k: &K, d: Duration) -> Option<Arc<V>> {
        let mut map = self.map.lock().unwrap();
        let option_v = map.get_mut(k);

        if let Some(v) = option_v {
            v.0 = Instant::now() + d;
            return Some(Arc::clone(&v.1));
        } else {
            return None;
        }
    }

    pub fn get(&self, k: &K) -> Option<Arc<V>> {
        let map = self.map.lock().unwrap();
        let option_v = map.get(k);
        if let Some(v) = option_v {
            return Some(Arc::clone(&v.1));
        } else {
            return None;
        }
    }
    
}

fn cache() {
    let mut cache: Cache<String, String> = Cache::new();
    let astring = "Nunzio".to_string();
    cache.put("Nunzio".to_string(), "Compleanno".to_string(), Duration::from_secs(4));
    cache.renew(&astring, Duration::from_secs_f64(100000.0));
    let event = cache.get(&astring);
    println!("{:?}",event);
    assert_eq!(cache.size(), 1);
}

pub fn main() {
    mutex();
    polymorphism();
    cache();
}