// Domanda 1: Si definiscano le principali aree di memoria associate ad un eseguibile e si mostri, attraverso opportuni
// esempi di codice, in quale situazione ciascuna di esse viene utilizzata.
//
// Le zone di memoria associate ad un eseguibile sono: stack, heap, variabili globali, costanti e
// codice. Per esempio:
// //VARIABILE GLOBALE
// static R: usize = 4;
// //COSTANTE
// const PI: f32 = 3.14;
//
// pub fn main() {
//     //HEAP
//     let mut computed: Vec<f32> = Vec::new();
//     //STACK
//     let area = PI * (R * R) as f32;
//
//     //CODICE
//     computed.push(area);
// }
// Domanda 2: Sia dato un primo valore di tipo std::cell::Cell<T> ed un secondo valore di tipo std::cell::RefCell<T>
// (dove T fa riferimento alla medesima entità). Si indichino le differenze tra i due e le modalità di
// occupazione della memoria (quantità, zone di memoria, ecc.).
//
// Il linguaggio rust consente al programmatore di anticipare un gran numero di errori in fase di compilazione piuttosto che in esecuzione.
// Per fare ciò tutte le variabili sono soggette ad alcune regole imposte da un modulo specifico
// del compilatore chiamato "borrow checker". Questo impone che ogni variabile segua 3 regole:
// 1. Ogni variabile può essere posseduta da un solo scopo.
// 2. Ogni variabile può avere multipli riferimenti in lettura se e solo se non esistono
//    riferimenti mutabili per quella variabile.
// 3. Ogni variabile può avere un solo riferimento mutabile se e solo se non esistono riferimenti
//    in lettura per quella variabile.
//
// Tornando alla domanda il tipo Cell consente al programmatore di ignorare la seconda e la terza regola
// elencata in fase di compilazione e rimandandone il controllo in fase di esecuzione.
// Mentre il tipo RefCell serve a rimandare in fase di esecuzione il controllo dell'effettivo
// lifetime di un riferimento passato a refcell.
// Per consentire ciò il tipo Cell prende possesso del dato passato al costruttore, effettua una copia del dato sullo stack 
// e offre metodi per la modifica del dato tramite get() e set().
// Il tipo RefCell acquisisce il riferimento immutabile ad un dato e offre al programmatore i
// metodi borrow() e borrow_mut() per ottenere riferimenti nuovi al dato in questione. Da notare
// che essendo il check rimandato in fase di esecuzione è responsabilità del programmatore non
// invocare un metodo finchè il lifetime del riferimento prodotto dall'altro è ancora valido e
// viceversa.
// In memoria RefCell viene rappresentato con la coppia di dati borrow, che indica se il dato è
// stato già prestato ed un riferimento immutabile al dato, entrambi i campi sono allocati sullo
// stack.
//
// Domanda 3: In un programma che utilizza una sincronizzazione basata su Condition Variable, è possibile che
// alcune notifiche vengano perse? Se sì, perché? In entrambi i casi si produca un esempio di codice
// che giustifichi la risposta.
// 
// Se il codice è scritto usando il metodo wait() è possibile che si verifichino due condizioni
// particolari in cui le notifiche non vengono "registrate" dal thread destinatario, in letteratura
// questi due eventi sono chiamati notifiche spurie e notifiche perse.
// Dati tre o più thread che rappresentano logicamente un mittente ed due o più destinatari di un dato
// il caso delle notifiche spurie o false notifiche si verifica quando uno dei destinatari verifica la condizione imposta dalla condition variable
// ed entra nella sezione critica per effettuare il lock e durante questa fase un altro
// destinatario più veloce acquisisce il lock, legge/consuma il dato e rilascia il lock, lasciando
// così il "primo" destinatario nella condizione di una falsa notifica.
// Un meccanismo introdotto per risolvere questo tipo di problema è offerto
// da rust tramite il metodo wait_while() delle condition variable che controlla la condizione di
// acquisizione del lock prima e dopo dell'acquisizione del lock sul dato e se il secondo controllo non ha successo non esegue la sezione critica che lavora sul dato.
//
// Una seconda tipologia di notifica non registrata dal destinatario è quella delle notifiche
// perse. Una notifica persa si ha quando il mittente è particolarmente veloce
// ed invia una seconda (o anche terza, o più) notifica mentre il destinatario sta ancora
// processando la prima notifica. Per limitare questo comportamento la struct Condvar offre un
// metodo wait_timeout(duration: Duration) in cui è possibile specificare un tempo massimo per
// l'esecuzione del controllo della condizione in modo da non spendere troppo tempo e limitare il
// numero di notifiche potenzialmente perse.
//
// Data la possibilità che entrambi gli eventi si verifichino in una stessa codebase esiste anche
// un metodo wait_while_timeout() che combina entrambi gli approcci per limitare la possibilità di
// false notifiche e notifiche perse.
//
// Domanda 4: In un sistema concorrente, ciascun thread può pubblicare eventi per rendere noto ad altri thread
// quanto sta facendo.
// Per evitare un accoppiamento stretto tra mittenti e destinatari degli eventi, si utilizza un
// Dispatcher: questo è un oggetto thread-safe che offre il metodo 
//  dispatch(msg: Msg) 
// mediante il quale un messaggio di tipo generico Msg (soggetto al vincolo di essere clonabile)
// viene reso disponibile a chiunque si sia sottoscritto. Un thread interessato a ricevere messaggi
// può invocare il metodo 
//  subscribe() 
// del Dispatcher: otterrà come risultato un oggetto di tipo Subscription mediante il quale potrà
// leggere i messaggi che da ora in poi saranno pubblicati attraverso il Dispatcher. Per ogni
// sottoscrizione attiva, il Dispatcher mantiene internamente l'equivalente di una coda ordinata
// (FIFO) di messaggi non ancora letti. A fronte dell'invocazione del metodo dispatch(msg:Msg), il
// messaggio viene clonato ed inserito in ciascuna delle code esistenti.
// L'oggetto Subscription offre il metodo bloccante 
//  read() -> Option<Msg>
// se nella coda corrispondente è presente almeno un messaggio, questo viene rimosso e restituito;
// se nella coda non è presente nessun messaggio e il Dispatcher esiste ancora, l'invocazione si
// blocca fino a che non viene inserito un nuovo messaggio; se invece il Dispatcher è stato distrutto,
// viene restituito il valore corrispondente all'opzione vuota.
// Gli oggetti Dispatcher e Subscription sono in qualche modo collegati, ma devono poter avere cicli
// di vita indipendenti: la distruzione del Dispatcher non deve impedire la consumazione dei
// messaggi già recapitati ad una Subscription, ma non ancora letti; parimenti, la distruzione di una
// Subscription non deve impedire al Dispatcher di consegnare ulteriori messaggi alle eventuali altre
// Subscription presenti.
// Si implementino le strutture dati Dispatcher e Subscription, a scelta, nel linguaggio Rust o C++11.
use std::sync::mpsc::{ Sender, Receiver, channel };
use std::ops::DerefMut;

#[derive(Clone)]
struct Msg<T: Clone> {
    message: T
}

struct Dispatcher<T: Clone + DerefMut>{
    senders_subs: Vec<Sender<Msg<T>>>, 
}
//trait `DerefMut` is required to modify through a dereference, but it is no
//t implemented for `Arc<Dispatcher<String>>`

impl<T:Clone + DerefMut> Dispatcher<T> {

    pub fn new() -> Dispatcher<T> {
        Dispatcher {
            senders_subs: Vec::new()
        }
    }

    pub fn dispatch(&mut self, msg: Msg<T>) {
        for sender in self.senders_subs.iter_mut() {
            let _ = sender.send(msg.clone());
        }
    }

    pub fn subscribe(&mut self) -> Subscription<T> {
        let (sender, receiver) = channel::<Msg<T>>();
        self.senders_subs.push(sender);
        Subscription {
            receiver
        }
    }

}

struct Subscription<T: Clone> {
    receiver: Receiver<Msg<T>>,
}

impl<T: Clone> Subscription<T> {

    pub fn read(&self) -> Option<Msg<T>> {
        if let Ok(msg) = self.receiver.recv() {
            return Some(msg);
        } else {
            return None;
        }
    }

}

use std::thread::{ sleep, spawn };
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;


pub fn main() {
    let dispatcher = Arc::new(Mutex::new(Dispatcher::new()));
    let sender_clone = Arc::clone(&dispatcher);
   
    let sender_handle = spawn(move || {
        println!("Sender is sending the message");
        let mut sender_clone = sender_clone.lock().unwrap();
        let msg = Msg {
            message: "Hi from the sender!".to_string()
        };
        sender_clone.dispatch(msg);
    });

    

    let receiver_handle = spawn(move || {  
        let mut dispatcher = dispatcher.lock().unwrap();
        let sub = dispatcher.subscribe();
        println!("Receiver has been sat up");
        if let Some(msg) = sub.read() {
            println!("{}",msg.message);
        }
    }); 

    let _ = sender_handle.join();
    let _ = receiver_handle.join();
}

#[cfg(test)]
mod test {
    use std::thread::{ sleep, spawn };
    use std::sync::Arc;
    use crate::{ Msg, Dispatcher };
    use std::sync::Mutex;
    use std::time::Duration;

    #[test]
    fn simple_test() {
        let dispatcher = Arc::new(Mutex::new(Dispatcher::new()));
        let sender_clone = Arc::clone(&dispatcher);
        
        let receiver_handle = spawn(move || {
            let mut dispatcher = dispatcher.lock().unwrap();
            let sub = dispatcher.subscribe();
            println!("Receiver has been sat up");
            if let Some(msg) = sub.read() {
                println!("{}",msg.message);
            }
        });

        sleep(Duration::from_millis(4000));

        let sender_handle = spawn(move || { 
            let mut sender_clone = sender_clone.lock().unwrap();
            let msg = Msg {
                message: "Hi from the sender!".to_string()
            };
            println!("Sender is sending the message");
            sender_clone.dispatch(msg);
        });

        let _ = sender_handle.join();
        let _ = receiver_handle.join();
    }
}
