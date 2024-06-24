//Domanda 1: Problema delle dipendenze cicliche
//Il problema delle dipendenze cicliche è un problema comune in Rust dato dal modo in cui Rust
//dealloca le strutture dalla memoria. Contrariamente al pattern RAII presente in C++, Rust offre
//al programmatore la possibilità di deallocare automaticamente le variabili quando lo scopo in cui
//sono dichiarate finisce. Tuttavia vi sono delle situazioni in cui il programmatore può desiderare
//che le strutture dati sopravvivano ad un determinato scopo e vengano condivise con altri thread
//tramite riferimenti o semplicemente una struttura possiede internamente dei riferimenti a se
//stessa. Ad esempio nel caso in cui il programmatore voglia creare una struttura dati e
//passarla per riferimento immutabile ad una funzione facendo si che quella funzione poi si faccia
//carico di modificarne un campo interno, ci si trova in una delle situazioni sopracitate chiamata
//mutabilità interna. Rust offre un particolare tipo di struttura chiamata Rc (Reference counter)
//che in coppia con Cell consente di risolvere il problema della mutabilità interna.
//Quando però la struttura ha finito di essere usata e va fuori dallo scopo il compilatore
//individua la presenza ancora di riferimenti alla struttura (Rc) e quindi non la dealloca.
//La soluzione a questo problema si ha tramite l'utilizzo di un conteggio delle occorrenze della
//struttura debole tramite il costrutto Weak. Mentre Rc mantiene un conteggio dei riferimenti alla
//struttura e consente di deallocarla quando il conteggio raggiunge 0, Weak consente un cosidetto
//"conteggio debole" che non conta il numero di riferimenti alla struttura nel momento in cui la
//stessa esce dallo scopo bensì utilizza il conteggio aggiornato all'ultimo riferimento creato.
//E' possibile creare un Weak a partire da Rc invocando il metodo downgrade() (ritorna un
//Option<Weak>) o ottenere un Rc a partire da Weak tramite il metodo upgrade().
//E' inoltre possibile trovare la stessa modalità di funzionamento per la struttura Arc (Atomic ref
//counter) che può essere tramutata in Weak e viceversa per risolvere l'altro scenario già
//presentato ovvero una struttura dati che necessita di fornire a più di un thread un riferimento
//a se stessa e ognuna dei riferimenti forti (Arc::clone) contiene internamente un puntatore a se
//stessa, prevenendo cosi' la deallocazione della struttura.
use std::{rc::{ Rc, Weak }, sync::{Arc, Condvar, Mutex}, thread::spawn};

fn weak_example() {
    let rc = Rc::new(5);
    let weak = Rc::downgrade(&rc);

    for _ in 0..5 { 
        let weak_clone = Weak::clone(&weak);
        dbg!("{} {}",Rc::strong_count(&rc),Weak::weak_count(&weak_clone));
    }
    dbg!("{} {}",Rc::strong_count(&rc),Weak::weak_count(&weak));
}

// Domanda 2: Una barriera è un costrutto di sincronizzazione usato per regolare l'avanzamento relativo della computazione di più thread. 
// All'atto della costruzione di questo oggetto, viene indicato il numero N di thread coinvolti. 

// Non è lecito creare una barriera che coinvolga meno di 2 thread. 

// La barriera offre un solo metodo, wait(), il cui scopo è bloccare temporaneamente l'esecuzione del thread che lo ha invocato, non ritornando fino a che non sono giunte 
// altre N-1 invocazioni dello stesso metodo da parte di altri thread: quando ciò succede, la barriera si sblocca e tutti tornano. Successive invocazioni del metodo wait() 
// hanno lo stesso comportamento: la barriera è ciclica.

// Attenzione a non mescolare le fasi di ingresso e di uscita!

// Una RankingBarrier è una versione particolare della barriera in cui il metodo wait() restituisce un intero che rappresenta l'ordine di arrivo: il primo thread ad avere 
// invocato wait() otterrà 1 come valore di ritorno, il secondo thread 2, e così via. All'inizio di un nuovo ciclo, il conteggio ripartirà da 1.

// Si implementi la struttura dati RankingBarrier a scelta nei linguaggi Rust o C++ '11 o successivi.
#[derive(Debug)]
struct BarrierState {
    size: usize,
    arrived: Vec<usize>,
    checked: Vec<usize>,
    ready: bool
}

impl BarrierState {

    pub fn new(size: usize) -> Option<Self> {
        if size < 2 {
            return None;
        }

        Some(BarrierState { size, arrived: Vec::new(), checked: Vec::new(), ready: false })
    }

    pub fn arrive(&mut self, id: usize) -> bool {
        if self.arrived.len() < self.size {
            self.arrived.push(id);
            self.ready = self.arrived.len() == self.size;
            return self.ready;
        }
        return false;
    }

    pub fn depart(&mut self, id: usize) -> Option<usize> {
        if self.ready {
            let arrived = self.arrived.iter().position(|el| {*el == id});
            if let Some(arrived) = arrived {
                self.checked.push(arrived);
                if self.checked.len() == self.arrived.len() {
                    self.arrived.clear();
                    self.checked.clear();
                    self.ready = false;
                }
                return Some(arrived);
            }
        }
        return None;
    }

}

struct RankingBarrier {
    lock: Mutex<BarrierState>,
    condvar: Condvar
}

impl RankingBarrier {

    pub fn new(size: usize) -> Option<RankingBarrier> {
        let state = BarrierState::new(size);
        if let Some(state) = state {
            return Some(RankingBarrier {
                lock: Mutex::new(state),
                condvar: Condvar::new()
            });
        } else {
            return None;
        }
    }

    pub fn wait(&self) -> usize {
        let id = {
            let mut state = self.lock.lock().unwrap();
            let id = state.arrived.len();
            let _ = state.arrive(id);
            id
        };
        let arrival = {
            let mut guard = self.condvar.wait_while(
            self.lock.lock().unwrap(),
            |state| {
                !state.ready
            }).unwrap();
            if let Some(arrival) = guard.depart(id) {
                self.condvar.notify_all();
                arrival + 1
            } else {
                0
            }
        };
        return arrival;
    }

}

fn ranking_barrier() {
    let barrier = Arc::new(RankingBarrier::new(4).unwrap());
    let mut handles = vec![];

    for i in 0..4 {
        let barrier_clone = Arc::clone(&barrier);
        let handle = spawn(move || {
            println!("THREAD {}: Ready to operate", i);
            let arrival_order = barrier_clone.wait();
            if arrival_order != 0 {
                println!("THREAD {}: Returned after arriving {}", i, arrival_order);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join();
    }

    handles = vec![];

    for i in 0..4 {
        let barrier_clone = Arc::clone(&barrier);
        let handle = spawn(move || {
            println!("THREAD {}: Ready to operate", i);
            let arrival_order = barrier_clone.wait();
            if arrival_order != 0 {
                println!("THREAD {}: Returned after arriving {}", i, arrival_order);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join();
    }
}

//Domanda 3: Nel caso di programmazione concorrente in Rust e memoria condivisa quali sono i
//principali costrutti e come vengono utilizzati per evitare la condizione di deadlock?
//
//Data la definizione di programmazione concorrente ovvero un insieme di thread che viene eseguito
//parallelamente ed un vincolo di sincronizzazione tra i thread la condizione di deadlock è un
//attesa indefinita da parte dei thread o di una parte di essi data da un operazione che non viene
//eseguita o la cui esecuzione non è stata "registrata" dagli altri thread in esecuzione.
//Questa condizione può essere evitata seguengo due tipologie di sincronizzazione offerte dalla
//libreria standard di rust: Sincronizzazione basata su uno stato condiviso (aggiornamento di un
//dato al termine dell'esecuzione di un task da parte di uno dei thread) o sincronizzazione basata
//su messaggi.
//Esistono anche librerie esterne alla standard come tokio, actix e molte altre che offrono
//meccanismi di sincronizzazione basati su altri approcci.
//Per quanto riguarda la sincronizzazione basata su uno stato comune rust offre costrutti per
//implementare velocemente l'acquisizione e il rilascio di uno stato sul quale uno dei thread vuole
//effetturare una lettura o una scrittura. I costrutti di cui sopra sono: Mutex, RwLock e Condvar.
//Questi possono essere usati singolarmente o anche insieme. Mutex consente ad un thread di
//attendere che la risorsa/stato da esaminare/modificare non sia usata da altri tramite il metodo
//lock(), RwLock è sostanzialmente un costrutto che ha la stessa funzionalità di Mutex ma offre una
//distinzione sul tipo di blocco presente sullo stato, è possibile vincolare l'utilizzo da parte
//dei thread dello stato condiviso ad una acquisizione singola solamente alla scrittura mentre la
//lettura può essere effettuata da tutti e contemporaneamente ma impedisce a chiunque di scrivere
//mentre almeno uno dei thread legge.
//Vi sono tra i meccanismi illustrati anche le condition variables o Condvar in Rust che offrono al
//programmatore la possibilità di esplicitare quale condizione i thread devono verificare e attendere 
//prima di operare sullo stato condiviso, queste spesso sono usate in coppia con uno dei tipi di
//lock precedentemente illustrati. Inoltre nella condivisione di uno stato è importante notare che
//Rust offre un costrutto chiamato Arc che non è prettamente inerente alla programmazione
//concorrente in generale ma risolve un vincolo che Rust stesso impone al programmatore per
//semplificare il processo di scrittura del codice, ovvero Rust obbliga il codice affinchè sia
//compilato ad avere un solo "proprietario" per ogni variabile, e con un reference counter si
//svincola il dato da questo vincolo delegando la deallocazione poi del dato quando il conteggio
//delle "copie" del dato si esaurisce. Inoltre Arc, atomic reference counter, assicura che il dato
//venga operato indivisibilmente ovvero le operazioni sul dato non presentino lo stesso in stati
//intermedi durante la modifica in modo che altri thread non possano effettuare letture "sbagliate".
//
//Per quanto riguarda invece i meccanismi di sincronizzazione basati sui messaggi Rust offre dei
//costrutti basati sul concetto di canale, Sender e Receiver, inoltre sfrutta anche l'uso dei
//tratti Sync e Send (usati in generale anche nella condivisione dello stato). Un canale è una
//funzione contenuta ed implementata come closure all'interno della libreria standard, esso
//mantiene una coda dei messaggi inviati. Un Sender è una struttura che consente l'invio dei
//messaggi sul canale ed implementa il tratto Send quindi può essere anche trasferita da un thread
//che crea il Sender ad un altro thread. Un Receiver invece consente di attendere in maniera
//bloccante l'arrivo di un messaggio e non implementa il tratto Send quindi non può essere
//trasferito da un thread ad un altro ma deve essere utilizzato dal thread che crea il canale.
//I tratti sopracitati Sync e Send vengono implementati direttamente dal compilatore e definiscono
//come già detto la trasferibilità in "sicurezza" del dato tra i thread, i dati che implementano
//Sync non possono essere trasferiti mentre quelli che implementano Send possono. Tutti i tipi di
//dato primitivi implementano Send e le strutture composte solo da dati primitivi "ereditano" questo tratto.
//Le strutture che implementano Send possono implementare anche il tratto Sync a patto che queste
//non contengano puntatori che possono creare errori nella sincronizzazione tra thread diversi
//quali le strutture che implementano meccanismi per la mutabilità interna, contatori di
//riferimenti non atomici ed altri tipi particolari di smart pointer.
//
//
//
//
//Domanda 4: Come si utilizza il sistema di gestione dei processi in Rust attraverso il pacchetto
//std::process? Quali sono le funzionalità offerte dalla libreria e come si gestiscono i processi
//figli nell'ambito del proprio programma?
//
//L'utilizzo dei processi ricade nella soluzione di un problema specifico, simile a quello
//presentato dalla programmazione concorrente ma non esattamente uguale. I processi infatti
//presentano limitazioni in termini di performance rispetto ai thread e quindi sono una soluzione
//in termini di parallelismo inferiore alla programmazione concorrente se l'obbiettivo del
//programmatore è quello di eseguire tanti task indipendenti o quasi tra loro in maniera parallela.
//Tuttavia questa soluzione è mirata ad un problema specifico e molto diffuso quale, la presenza di
//programmi paralleli che necessitano frequentemente di sincronizzazione e quindi che spesso
//"attendono" in parallelo per citare la letteratura informatica. I meccanismi di sincronizzazione
//tra i processi sono basati sull'utilizzo di filesystem in quanto avendo i processi strutture dati
//che possiedono indirizzi logici queste non possono essere condivise tramite puntatori. L'insieme dei
//metodi utilizzati per la comunicazione tra questi viene generalmente riferico con l'acronimo IPC
//che riassume appunto il termine Inter Process Comunication. Rust offre al programmatore che
//necessita di programmare asincronamente una grande varietà di strumenti affinchè l'esperienza
//dello sviluppatore venga semplificata e il numero di errori che esso può commettere sia limitato
//definendo così il concetto di Fearless Concurrency, che viene però maggiormente usato come
//riferimento per i metodi della programmazione concorrente.
//Il mezzo principale che Rust offre al programmatore è dato dalla presenza del tratto Future,
//della keyword async e del metodo await(). Questi costrutti appena presentati rappresentano la
//base per la programmazione asyncrona. Una funzione che implementa il tratto Future corrisponde
//logicamente ad una macchina a stati finiti che in base al risultato di un operazione può cambiare
//il risultato delle operazioni da essa eseguite. Se una funzione viene annotata precedendo la sua
//dichiarazione con la keyword async essa viene automaticamente marcata per poi implementare il
//tratto Future. Tramite il metodo await() invece, è possibile attendere che un'operazione
//asincrona termini bloccando l'esecuzione di operazione successive fino al termine di questa.
//Data questa introduzione Rust offre anche costrutti per l'avvio di processi tramite interazione
//con il sistema operativo tramite la struttura dati Command. Questa permette di invocare programmi
//già esistenti sul device su cui viene eseguito il programma attendendone la terminazione,
//verificandone lo stato di successo e/o catturandone il risultato. Tramite i metodi stdout, stderr
//e stdin e possibile indicare se in seguito all'avvio del processo tramite la struct Command
//l'input e l'output del processo debbano essere ereditati dal processo corrente, debbano essere
//ignorati o debbano essere reindirizzati ad un processo genitore. La enum Stdio specifica questi
//comportamenti tramite le opzioni Inherit, Null e Pipe.
//Il metodo status permette di attendere e verificare il successo del processo avviato, se ha
//scatenato errori o no, mentre il metodo output permette di attendere l'esecuzione e raccogliere
//l'output del processo.

pub fn main() {
    weak_example();
    println!("------------------");
    ranking_barrier();
    println!("------------------");
}
