// Domanda 1: Si spieghi il concetto di possesso in relazione agli smart pointers. Come viene gestito il ciclo delle
// risorse quando si utilizzano smart pointers?
// Infine, si espliciti il ciclo di vita delle risorse nel seguente esempio:
// pub fn main() {
//     let mut i = 10;
//     let bi1 = Box::new(i);
//     let mut bi2 = Box::new(*bi1);
//     *bi2 = 20;
//     i = *bi2;
//     println!("{} {:?} {:?}", i, bi1, bi2);
// }
//
// Il concetto di possesso nella programmazione prende senso quando si pensa alla memoria allocata
// sullo stack all'interno di un programma. Dati una serie di scopi uno contenente l'altro o
// indipendenti tra loro il possesso di una variabile corrisponde all'esistenza all'interno del
// proprio stack di un riferimento alla zona di memoria in cui il valore interno alla variabile è
// stato scritto. I problemi nascono quando viene fatto un uso improprio degli indirizzi di memoria
// perciò gli smart pointers provano a risolvere sul nascere gli usi impropri di questi indirizzi.
// L'implementazione interna degli smart pointer segue due possibili approcci in Rust, quello di
// copiare il dato e delegarne il possesso allo smart pointer e quello di leggere il dato e
// allocarne una copia ma sullo heap. Nel esempio sopra presentato tramite l'uso di Box si alloca
// il suo contenuto i (10) sullo heap, quando si alloca sullo heap bi2 questo prende come parametro
// il risultato dell'implementazione di Deref su bi1 che corrisponde all'indirizzo dello heap su
// cui è stato posizionato 10. Quando invece si dereferenzia in maniera mutabile bi2 si cambia
// il suo contenuto al valore 20! Se in seguito si assegna questo valore ad i si ottiene
// il valore 20.
// Quindi il risultato di println sarà 20, Box(10), Box(20).
// Dal punto di vista del ciclo di vita di queste variabili essendo tutte allocate sullo stesso
// scopo senza essere ritornate la loro deallocazione avverrà al termine dello scopo stesso. Da
// notare che se invece di Box fosse stato usato un puntatore nativo si generebbe un wild pointer
// all'istruzione "*bi2 = 20;".
//
// Domanda 2: Si descriva la gestione della memoria in Rust e si spieghi come vengono evitati i problemi di
// sicurezza comuni come le violazioni di accesso o la presenza di puntatori nulli.
//
// Rust implementa un sistema interno al suo compilatore che prima di permettere la compilazione
// verifica una serie di regole delegate ad una componente funzionale del compilatore chiamata
// borrow checker. Inoltre l'uso dei puntatori nativi viene limitato ad una speciale notazione
// creando uno scopo preceduto dalla keyword unsafe dove il compilatore viene inibito nel controllo
// del borrow checker.
// Nella versione "safe" del linguaggio quindi il controllo e la gestione della memoria sono
// relegati all'uso dei costrutti standard esistenti quali i riferimenti immutabili e mutabili ed
// ai controlli del borrow checker.
// La distinzione tra i due tipi di riferimento introduce un interessante spunto di riflessione
// sulla filosofia del linguaggio in quanto il loro utilizzo internamente nasconde le stesse
// informazioni sull'indirizzo di memoria a cui un dato si trova ma forza il programmatore a
// dichiarare esplicitamente l'uso che vuole fare del dato permettendo cosi' il controllo del
// borrow checker che ha vita facile nel capire se le operazioni effettuate sono lecite o no.
// Le regole imposte e controllate come già introdotto sono le seguenti:
// - Una variabile può essere posseduta da un solo scopo.
// - In uno stesso scopo ossono esistere contemporaneamente più riferimenti immutabili ad una variabile ma non
// riferimenti mutabili.
// - In uno stesso scopo può esistere un riferimento mutabile ad una variabile ma nessun riferimento immutabile.
//
// Domanda 3: Si illustri come sia possibile gestire correttamente le situazioni di errore in Rust, distinguendo tra
// Option e Result.
//
// Data l'esistenza di algoritmi che non necessariamente terminano correttamente (per input errati
// o situazioni imprevedibili relative all'esecutore) Rust fornisce dei costrutti al programmatore
// per gestire queste casistiche. L'enum Option si utilizza per indicare che un tipo di dato può
// essere null senza effettivamente avere puntatori ad un valore "nullo". Questo costrutto inoltre
// è dotato di molti metodi per evitare operazioni sul dato se esso è nullo come metodi di
// controllo su option .is_some() e .is_none(). Vi sono meccanismi per l'accesso diretto al
// contenuto dell'option senza scatenare errori come if let Some(...) = option_value {}.
// Allo stesso modo è possibile usare Result come tipo di dato sebbene indichi solo il risultato di
// un operazione non un dato ritornato da essa. Result viene fornito con un set di metodi quali:
// is_ok(), is_err(), ok(), err() etc. Supporta lo stesso costrutto illustrato con option, if let
// Ok(...) = result_value {} e inoltre permette l'utilizzo del costrutto ? all'interno delle
// funzioni annotate con un risultato di tipo Result<T,E> per implementare velocemente l'esecuzione
// di metodi se e solo se questi vengono implementati su un result che ha avuto status Ok.
// Vi sono inoltre all'interno dell'ecosistema di crates disponibili in cargo, crates esterni al
// linguaggio standard che forniscono funzionalità aggiuntive come thiserror e anyhow.
//
// Domanda 4: Una DelayedQueue<T:Send> è un particolare tipo di coda non limitata che offre tre metodi
// principali, oltre alla funzione costruttrice:
// 1. offer(&self, t:T, i: Instant): Inserisce un elemento che non potrà essere estratto prima
// dell'istante di scadenza i.
// 2. take(&self) -> Option<T>: Cerca l'elemento t con scadenza più ravvicinata: se tale
// scadenza è già stata oltrepassata, restituisce Some(t); se la scadenza non è ancora stata
// superata, attende senza consumare cicli di CPU, che tale tempo trascorra, per poi restituire
// Some(t); se non è presente nessun elemento in coda, restituisce None. Se, durante l'attesa,
// avviene un cambiamento qualsiasi al contenuto della coda, ripete il procedimento suddetto
// con il nuovo elemento a scadenza più ravvicinata (ammesso che ci sia ancora).
// 3. size(&self) -> usize: restituisce il numero di elementi in coda indipendentemente dal fatto
// che siano scaduti o meno.
// Si implementi tale struttura dati nel linguaggio Rust, avendo cura di renderne il comportamento
// thread-safe. Si ricordi che gli oggetti di tipo Condvar offrono un meccanismo di attesa limitata nel
// tempo, offerto dai metodi wait_timeout(...) e wait_timeout_while(...)).
use std::sync::{ Mutex, Condvar };
use std::time::{ Instant, Duration };

struct DelayedQueue<T: Send + Copy> {
    queue: Mutex<Vec<(T, Instant)>>,
    condvar: Condvar,
}

impl<T: Send + Copy> DelayedQueue<T> {

    pub fn new() -> DelayedQueue<T> {
        DelayedQueue {
            queue: Mutex::new(Vec::new()),
            condvar: Condvar::new(),
        }
    }
    
    pub fn offer(&self, t: T, i: Instant) {
        let now = Instant::now();
        let mut queue = self.condvar.wait_timeout_while(self.queue.lock().unwrap(), i - now , |_| false ).unwrap().0;
        queue.push((t,i));
    }

    pub fn take(&self) -> Option<T> {
        let now = Instant::now();
        let mut queue = self.queue.lock().unwrap();
        queue.sort_by(|(_,duetime),(_,duetime2)| duetime.partial_cmp(duetime2).unwrap());
        let nearest = queue.first();
        if let Some(nearest) = nearest {
            if nearest.1 < Instant::now() {
                let mut queue = self.condvar.wait_while(self.queue.lock().unwrap(), |_| nearest.1.duration_since(now) <= Duration::from_secs(0));
            }
            let item = queue.remove(0);
            return Some(item.0);
        } else {
            return None;
        }
    }

    pub fn size(&self) -> usize {
        let queue = self.queue.lock().unwrap();
        return queue.len();
    }
}

fn future_instant(seconds_delay: u64) -> Instant {
    use std::time::Duration;
    let duration = Duration::from_secs(seconds_delay);
    return Instant::now() + duration;
}


pub fn main() {
    let delayed_queue = DelayedQueue::new();
    
    for i in 0..5 {
        let item = (i, future_instant(5-i));
        delayed_queue.offer(item.0, item.1);
    }

    for i in 0..5 {
        let item = delayed_queue.take();
        if let Some(item) = item {
            println!("{}",item);
        }
    }
}
