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

use std::{collections::VecDeque, fmt::Debug, rc::{Rc, Weak}, sync::{Arc, Condvar, Mutex, RwLock}, thread::spawn};

fn cyclic() {
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
    Domanda 4: La struct MpMcChannel<E: Send> è una implementazione di un canale su cui possono scrivere molti produttori e da cui possono attingere valori molti consumatori.
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
struct CircularBuffer<E: Send> {
    v: Vec<Option<E>>,
    size: usize,
    head: usize,
    tail: usize,
    closed: bool,
}

impl<E: Send> CircularBuffer<E> {
    fn new(n: usize) -> Self {
        let mut v : Vec<Option<E>> = Vec::with_capacity(n);
        for i in 0..n {
            v.push(None);
        }

        Self{ v, size: 0, head: 0, tail: 0, closed: false }
    }

    fn is_empty(&self) -> bool {
        return self.size == 0;
    }

    fn is_full(&self) -> bool {
        return self.size == self.v.len();
    }

    fn push(&mut self, elem: E) {
        self.v[self.tail] = Some(elem);
        self.tail = (self.tail + 1) % self.v.len();
        self.size += 1;
    }
    
    fn pop(&mut self) -> E {
        let ret = self.v[self.head].take();
        self.head = (self.head + 1) % self.v.len();
        self.size -= 1;
        ret.unwrap()
    }

    fn is_closed(&self) -> bool {
        self.closed
    }

    fn close(&mut self) {
        self.closed = true;
    }
}

struct MpMcChannel<E: Send> {
    shared_data: Arc<(Mutex<CircularBuffer<E>>, Condvar)>
}

impl<E: Send> MpMcChannel<E> {

    fn new(n: usize) -> Self {
        let cb = CircularBuffer::new(n);
        Self{ shared_data: Arc::new((Mutex::new(cb), Condvar::new())) }
    }

    fn send(&self, e: E) -> Option<()> {
        let (mutex, cv) = &*self.shared_data;
        let mut cb_guard = cv.wait_while(mutex.lock().unwrap(), |cb| cb.is_full() && !cb.is_closed()).unwrap();
        
        if cb_guard.is_closed() {
         // the channel was closed while waiting or it was already closed, so don't insert
         None
        }
        
        else {
            // the channel was not closed so the wait ended because buffer is no longer full, so insert and notify
            cb_guard.push(e);
            cv.notify_all();
            Some(())
        }
    }

    fn recv(&self) -> Option<E> {
        let (mutex, cv) = &*self.shared_data;
        let mut cb_guard = cv.wait_while(mutex.lock().unwrap(), |cb| cb.is_empty() && !cb.is_closed()).unwrap();

        if cb_guard.is_empty() {
            // means that !cb.is_closed() is false, so the channel is closed and there is no more data to read
            None
        } else {

        // means that there is some data to read, so we don't care if the channel is already closed
        // retrieve data and notify so that if someone is waiting to write he will be woken up
        let ret = cb_guard.pop();
            cv.notify_all();
            Some(ret)
        }
    }

    fn shutdown(&self) -> Option<()> {
        let (mutex, cv) = &*self.shared_data;
        let mut cb_guard = mutex.lock().unwrap();
        cb_guard.close();
        cv.notify_all();
        return Some(());
    }
}

fn mpmc_channel() {
    let mpmc_channel = Arc::new(RwLock::new(MpMcChannel::<usize>::new(4)));
    let mut receivers_handles = vec![]; 
    let mut senders_handles = vec![];

    for i in 0..10 {        
        let mpmc_channel_clone = Arc::clone(&mpmc_channel);
        let handle = spawn(move || {
            let mut guard = mpmc_channel_clone.write().unwrap();
            println!("Sender {}: READY", i);
            let res = guard.send(i);
            if res.is_some() {
                println!("Sender {}: SENT A MESSAGE TO ALL", i);
            } else {
                println!("Sender {}: UNABLE TO SEND THE MESSAGE", i);
            }
        });
        senders_handles.push(handle);
    }

    for i in 0..10 {
        let mpmc_channel_clone = Arc::clone(&mpmc_channel);
        let handle = spawn(move || {
            let guard = mpmc_channel_clone.read().unwrap();
            println!("Receiver {}: READY",i);
            let data = guard.recv();
            if let Some(data) = data {
                println!("Receiver {}: RECEIVED A MESSAGE FROM {}", i, data);
            } else {
                println!("Receiver {}: RECEIVED NOTHING", i);
            }
        });
        receivers_handles.push(handle);
    }


    
    for (i, handle) in receivers_handles.into_iter().enumerate() {
        let _ = handle.join();
        println!("Receiver {}: CLOSED",i);
    }

    for (i, handle) in senders_handles.into_iter().enumerate() {
        let _ = handle.join();
        println!("Sender {}: CLOSED",i);
    }
}


pub fn main() {
    cyclic();
    mpmc_channel();
}