/*
    Domanda 1: Si definisca il problema delle referenze cicliche nell’uso degli smart pointers.
    Si fornisca quindi un esempio in cui tale problema sia presente.

    -------------------------------------------------------------------

    Il problema delle dipendenze cicliche in generale si verifica quando una struttura dati contiene un riferimento a se stessa.
    In Rust questo problema porta un programma in genere a non poter deallocare correttamente la struttura in questione.
    Un esempio in cui questa condizione si verifica è quello della mutabilità interna o delle strutture dati ricorsive come le linked list o gli alberi.
    Rust offre costrutti per poter supportare la compilazione e il corretto comportamento di queste strutture dati tramite i costrutti Rc<T> e Weak<T>.
    La struttura Rc<T> consente di creare una copia di un dato e inizializzare il numero di riferimenti a 0. Quando vengono creati nuovi riferimenti al dato 
    tramite Rc::clone(&dato) esso incrementa il conteggio dei riferimenti. La struttura Weak viene derivata da Rc tramite il metodo downgrade e consente contrariamente a Rc
    di avere un conteggio basato non sulle copie create con Rc::clone ma a riferimenti veri e propri consentendo cosi' di evitare conteggi ricorsivi e deallocare automaticamente
    le strutture quando il conteggio di weak arriva a 0.
*/

use std::{fmt::Debug, rc::{Rc, Weak}, sync::{Arc, Condvar, Mutex}, thread::{sleep, spawn}, time::Duration};

fn _cyclic() {
    struct Node<T: Debug> {
        value: T,
        next: Option<Weak<Node<T>>>
    }

    impl<T: Debug> Debug for Node<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Node").field("value", &self.value).field("next", &self.next).finish()
        }
    }

    let el0 = Rc::new(Node {
        value: 10,
        next: None
    });
    let el1 = Node {
        value: 20,
        next: Some(Rc::downgrade(&el0))
    };

    println!("{:?}",el1);
}

/*
    Domanda 2: Data la struttura dati definita come:

    struct Data {
        Element: AsVector,
        next: Rc<Data>
    }

    enum AsVector {
        AsVector(Box::<Rc<i32>>),
        None
    }
    Indicare l’occupazione di memoria di un singolo elemento in termini di:

    numero di byte complessivi (caso peggiore) e
    posizionamento dei vari byte in memoria (stack, heap, ecc.)

    -------------------------------------------------------------------

    L'elemento singolo AsVector è una enum e quindi prende la dimensione di Box::<Rc<i32>>,
    a sua volta Box effettuando una copia di Rc non aggiunge overhead in termini di dimensioni del dato.
    Invece Rc contiene una copia del dato ed un conteggio dei riferimenti debole ed uno forte. Essendo il dato i32
    abbiamo 4 byte, per ogni conteggio verrà usato un usize che nel caso di un sistema a 64bit sarà pari a 8 byte
    per una dimensione totale del dato pari a 4+8+8 byte = 20 byte.

    Per quando riguarda la posizione in memoria dell'elemento singolo, l'enum AsVector viene memorizzata sullo stack
    come riferimento a Box che invece viene allocato sullo heap e conterrà un riferimento alla posizione di Rc che
    a sua volta si trova sullo heap.
*/

/*
    Domanda 3: Si identifichino i tratti fondamentali della concorrenza.
    Successivamente, in riferimento alla mutabilità/immutabilità delle risorse, si delinei come questa affligga
    la gestione della sincronizzazione a livello thread.

    -------------------------------------------------------------------

    I tratti forniti in Rust relativi alla gestione della concorrenza sono Future, Sync e Send.
    Ognuno di questi contribuisce alla compilazione di programmi eseguibili in maniera sicura.
    Infatti questi tratti sono automaticamente assegnati ai tipi di dato creati in un programma stabilendo rispettivamente:

    - Che le funzioni asincrone dichiarate tramite la keyword async implementino il metodo await() e possano essere quindi aspettate fino alla loro terminazione.
    Inoltre una funzione asincrona comporta che la funzione che la racchiude a sua volta debba essere marcata come asincrona se il metodo await non viene invocato.

    - Tramite il tratto Sync si stabilisce quali dati possono essere condivisi tra più thread nonostante possiedano riferimenti ad altri dati
    
    - Tramite il tratto Send si stabilisce quali dati possono essere condivisi tra più thread, tutti i dati con il tratto Sync devono implementare Send ma non viceversa.

    Per quanto riguarda la mutabilità/immutabilità dei dati all'interno di un programma multithreaded questa forzatura imposta dal compilatore ritorna utile.
    Infatti uno dei problemi comuni della programmazione concorrente è quello delle corse critiche, queste corrispondono all'accesso contemporaneo a sezioni di codice che invece
    richiederebbero l'accesso a delle risorse limitate/ che possono essere operate da un solo thread alla volta.
    La forzatura sull'assenza mutabilità/immutabilità contemporanea su di un dato fa si che il programmatore non si debba preoccupare dell'esistenza di accessi in scrittura durante le letture di un dato
    in quanto il borrow checker segnalerà errore in fase di compilazione del programma se questo avviene.
    Tuttavia è importante notare che per la flessibilità necessaria per alcune implementazioni Rust offre strutture opportune necessarie ad aggirare le regole sopra citate.

*/

/*
    Domanda 4: La struct MpMcChannel<E: Send> è una implementazione di un canale su cui possono scrivere molti produttori e
    da cui possono attingere valori molti consumatori.
    Tale struttura offre i seguenti metodi:

    new(n: usize) -> Self: crea un'instanza del canale basato su un buffer circolare di n elementi;
    
    send(e: E) -> Option<()>: invia l'elemento "e" sul canale. Se il buffer circolare è pieno, attende senza consumare CPU che
    si crei almeno un posto libero in cui depositare il valore.
    Ritorna: Some(()) se è stato possibile inserire il valore nel buffer circolare, None se il canale è stato chiuso
    (Attenzione: la chiusura può avvenire anche mentre si è in attesa che si liberi spazio) o se si è verificato un errore interno;
    
    recv() -> Option<E>: legge il prossimo elemento presente sul canale.
    Se il buffer circolare è vuoto, attende senza consumare CPU che venga depositato almeno un valore.
    Ritorna: Some(e) se è stato possibile prelevare un valore dal buffer,
    None se il canale è stato chiuso (Attenzione: se, all'atto della chiusura sono già presenti valori nel buffer,
    questi devono essere ritornati prima di indicare che il buffer è stato chiuso; se la chiusura avviene mentre si è in attesa di un valore,
    l'attesa si sblocca e viene ritornato None) o se si è verificato un errore interno;
    
    shutdown() -> Option<()>: chiude il canale, impedendo ulteriori invii di valori.
    Ritorna: Some(()) per indicare la corretta chiusura,
    None in caso di errore interno all'implementazione del metodo.
    
    Si implementi tale struttura dati in linguaggio Rust,
    senza utilizzare i canali forniti dalla libreria standard o da altre librerie,
    avendo cura di garantirne la correttezza in presenza di più thread e di non generare la condizione di panico all'interno dei suoi metodi.

    -------------------------------------------------------------------
    
*/

struct CircularBuffer<T: Clone> {
    buffer: Vec<T>,
    n: usize,
    size: usize,
    closed: bool
}

impl<T: Clone> CircularBuffer<T> {
    pub fn new(n: usize) -> CircularBuffer<T> {
        CircularBuffer {
            buffer: vec![],
            n,
            size: 0,
            closed: false
        }
    }

    pub fn put(&mut self, el: T) -> bool {
        if self.closed || self.size == self.n {
            return false;
        }
        self.buffer.push(el);
        self.size = self.size + 1;
        return true;
    }

    pub fn get(&mut self) -> Option<T> {
        if self.buffer.is_empty() {
            return None;
        }
        let el = self.buffer.pop();
        if el.is_some() {
            self.size = self.size - 1;
        }
        el
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn close(&mut self) -> Option<()> {
        if self.closed {
            return None;
        }
        self.closed = true;
        Some(())
    }

    pub fn is_closed(&self) -> bool {
        self.closed
    }

}

struct MpMcChannel<E: Sync + Clone> {
    lock: Mutex<CircularBuffer<E>>,
    condvar: Condvar,
}

impl <E: Sync + Clone> MpMcChannel<E> {

    pub fn new(n: usize) -> MpMcChannel<E> {
        MpMcChannel {
            lock: Mutex::new(CircularBuffer::new(n)),
            condvar: Condvar::new(),
        }
    }

    pub fn send(&self, el: E) -> Option<()> {
        let mut guard = self.condvar
        .wait_timeout_while(
            self.lock.lock().unwrap(),
        Duration::from_millis(100),
         |buffer| buffer.is_closed() || buffer.len() == buffer.n
        ).unwrap();
        if guard.1.timed_out() {
            return None;
        }
        if guard.0.put(el) {
            Some(())
        } else {
            None
        }
    }

    pub fn recv(&self) -> Option<E> {
        let mut guard = self.condvar
        .wait_timeout_while(
            self.lock.lock().unwrap(),
        Duration::from_millis(100),
         |buffer| buffer.len() == 0
        ).unwrap();
        if guard.1.timed_out() {
            return None;
        }
        guard.0.get()
    }

    pub fn shutdown(&self) -> Option<()> {
        let mut guard = self.condvar
        .wait_while(
            self.lock.lock().unwrap(),
         |buffer| !buffer.closed && buffer.len() == 0
        ).unwrap();
        guard.close();
        Some(())
    }
    
}

pub fn main() {
    let n = 5;
    let channel: Arc<Mutex<MpMcChannel<usize>>> = Arc::new(Mutex::new(MpMcChannel::new(n)));
    let mut handles = Vec::new();

    for i in 0..n {
        let channel = Arc::clone(&channel);
        let handle = spawn(move || {
            let guard = channel.lock().unwrap();
            let _ = guard.shutdown();
            let result = guard.send(i);
            if result.is_some() {
                println!("Sent: {:?}", i);
            }
        });
        handles.push(handle);
    }    

    sleep(Duration::from_secs(1));

    for _ in 0..n {
        let channel = Arc::clone(&channel);
        let handle = spawn(move || {
            let guard = channel.lock().unwrap();
            let el = guard.recv();
            println!("Received: {:?}", el);
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join();
    }
}