use std::fmt::Debug;
//Domanda 1: Si definisca il concetto di smart pointer, quindi si fornisca un esempio (Rust o C++)
//che ne evidenzi il ciclo di vita.
//
//Uno smart pointer è genericamente una struttura dati che avvolge un puntatore ad un dato.
//Mentre i normali puntatori normali rappresentano effettivamente la corrispondenza tra il dato e
//il suo indirizzo in memoria e qualsiasi operazione su di essi è consentita (con i conseguenti
//problemi), gli smart pointer si occupano di fornire al programmatore un set limitato di opzioni
//di esecuzione in modo che in grandi codebase la scrittura di codice da parte di persone
//differenti mantenga sintatticamente delle proprietà esplicite e riduca la possibilità di
//commettere errori conoscendo lo scopo per cui il puntatore è stato creato. In C++ gli smart
//pointers sono stati introdotti nella versione del linguaggio C++11, tra i primi smart pointers
//introdotti abbiamo unique_ptr e shared_ptr. Questi puntatori limitano la compilazione del codice
//alla presenza per un dato di un unico puntatore alla volta, quando lo scopo di un unique_ptr
//termina è possibile crearne uno nuovo o di uno shared_ptr che ad esempio permette di avere
//funzionalità analoghe ai puntatori nativi del linguaggio.
//In Rust invece data la filosofia del linguaggio orientata a limitare gli errori prima ancora che
//un programma venga compilato esistono diverse varietà di smart pointer ognuno orientato ad uno
//scopo diverso. Tra gli smart pointers più comuni abbiamo: Box<T>, Rc<T>, Weak<T>, Cell<T>, RefCell<T>,
//Cow<T> e Arc<T>.
//Lo smart pointer Box<T> ad esempio mira ad allocare un dato specifico sullo heap piuttosto che
//sullo stack, le motivazioni e quindi le funzionalità che mira ad offrire sono la sopravvivenza
//allo scopo in cui il dato viene creato e la corretta deallocazione del dato stesso al termine del
//programma. Molto spesso il tipo Box<T> viene usato in coppia con le virtual table che vengono
//create in fase di esecuzione per poter contenere dati di una specifico tipo riferendosi ad essi
//tramite tratti che implementano. Questo consente di implementare tramite Box meccanismi
//corrispondenti in altri linguaggi a comportamenti ereditari piuttosto che polimorfici, potendo
//cosi' raggruppare dati di diverso tipo indicando che i loro contenitori sono collezioni di
//Box<tratto comune dei dati>.
//
//Domanda 2: Si illustrino le differenze nel linguaggio Rust tra std::channel() e
//std::sync_channel(), indicando quali tipi di sincronizzazione i due meccanismi permettono.
//
//In rust la sincronizzazione tra thread differenti può avvenire tramite l'uso di meccanismi basati
//su messaggi. Ogni canale di comunicazione per l'invio dei messaggi è unidirezionale e viene
//creato con una delle due funzioni sopracitate. Alla creazione queste funzioni ritornano una
//struttura dati rappresentante il mittente e il destinatario per il canale, sotto forma di
//strutture rust Sender e Receiver. Queste implementano i metodi che verranno effettivamente usati
//per la comunicazione. Il Sender deve essere trasferito al thread che invierà i messaggi mentre il
//Receiver non può essere "mosso" dal thread che invoca la funzione di creazione del canale.
//La differenza sostanziale tra i due metodi di creazione dei canali sta nella capacità
//dei canali stessi che in qualche modo influenza la frequenza con cui mittente e
//destinatario dovranno interagire prima di comunicare nuovamente. Mentre la funzione channel()
//consente di conservare un numero illimitato di messaggi e quindi consente al Sender di inviare
//messaggi senza preoccuparsi che il destinatario li abbia ricevuti, la funzione sync_channel()
//richiede alla creazione di passarvi un parametro numerico che indicherà la capacità del canale.
//Quando il sender nel secondo caso invia un nuovo messaggio superando la capacità del canale
//specificata, esso entra in attesa che il canale si svuoti e che quindi data la capacità n che
//almeno un messaggio venga letto ogni n mandati. E' possibile implementare un canale di rendevouz
//passando a sync_channel() come parametro 0. Facendo ciò il sender prima di inviare ogni messaggio
//deve controllare che il messaggio precedente sia stato letto, altrimenti entra in blocco fino a
//che ciò non accade.
//
//Domanda 3: Dato il seguente codice Rust (ogni linea è preceduta dal suo indice) si descriva il
//contenuto dello stack e dello heap al termine dell'esecuzione della riga 15:
//
//"
// 1. struct Point { 
// 2. x: i16, 
// 3. y: i16,
// 4. }
// 5.
// 6. enum PathCommand { 
// 7. Move(Point), 
// 8. Line(Point),
// 9. Close,
// 10. }
// 11. let mut v = Vec::<PathCommand>::new();
// 12. v.push(PathCommand::Move(Point{x:1,y:1 }));
// 13. v.push(PathCommand::Line(Point{x:10, y:20}));
// 14. v.push(PathCommand::C/ose);
// 15. let slice = &v[ .. ];
//"
//
// Le definizioni della struct Point e dell'enum PathCommand non occupano memoria ma indicano
// quanta memoria ognuna delle instanze dei dati stessi occuperanno ovvero entrambe 4byte.
// 
// All'esecuzione della riga 11, il vettore viene allocato e sullo heap vengono allocati 8byte
// rappresentanti con i primi 4 la dimensione attuale del vettore che sarà quindi 0 e il puntatore
// all'indirizzo del primo elemento, che sarà inizialmente non settato quindi la memoria nei
// secondi 4 byte manterrà il valore che aveva prima dell'esecuzione del programma presentato.
// Nello stack invece verrà allocata la variabile v contenente l'indirizzo della struttura sullo
// heap e quindi occuperà 4 byte.
// Heap: 8 byte.
// Stack: 4 byte.
//
// All'esecuzione della riga 12, il vettore viene esteso con un elemento di PathCommand. La memoria
// dello heap cambia nel contenuto facendo puntare i secondi 4 byte del blocco al nuovo elemento
// aggiunto e cresce in dimensione di 4 byte per l'allocazione del nuovo elemento. Lo stack rimane
// invariato.
//
// Heap: 12 byte.
// Stack: 4 byte.
//
// All'esecuzione delle righe 13 e 14 si ripete lo stesso procedimento di 12 e quindi lo heap
// cresce ad ognuna delle esecuzioni di 4 byte e lo stack rimane invariato.
//
// Heap: 20 byte.
// Stack: 4 byte.
//
// All'esecuzione della riga 15, viene creata una slice del vettore precedentemente allocato il che
// implica la creazione di un puntatore sullo stack al vettore occupando 4 byte.
// Heap: 20 byte.
// Stack: 8 byte.
//
//
// Domanda 4: Un paradigma frequentemente usato nei sistemi reattivi e costituito dall'astrazione detta Looper. 
// Quando viene creato, un Looper crea una coda di oggetti generici di tipo Message ed un thread. 
// II thread attende - senza consumare cicli di CPU - che siano presenti messaggi nella coda, Ii 
// estrae a uno a uno nell'ordine di arrivo, e li elabora. II costruttore di Looper riceve due parametri, 
// entrambi di tipo (puntatore a) funzione: process( ... ) e cleanup(). La prim a e una funzione
// responsabile di elaborare i singoli messaggi ricevuti attraverso la coda; tale funzione accetta un
// unico parametro in ingresso di tipo Message e non ritorna nulla; La seconda e funzione priva di 
// argomenti e valore di ritorno e verra invocata dal thread incapsulato nel Looper quando esso 
// stara per terminare. 
// Looper offre un unico metodo pubblico, thread safe, oltre a quelli di servizio, necessari per 
// gestirne ii ciclo di vita: send(msg), che accetta come parametro un oggetto generico di tipo 
// Message che verra inserito nella coda e successivamente estratto dal thread ed inoltrato alla 
// funzione di elaborazione. Quando un oggetto Looper viene distrutto, occorre fare in modo che ii 
// thread contenuto al suo interno invochi la seconda funzione passata nel costruttore e poi termini. 
// Si implementi, utilizzando ii linguaggio Rust o C++, tale astrazione tenendo canto che i suoi metodi 
// dovranno essere thread-safe.
//
use std::sync::{Arc, Condvar, Mutex};
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread::{self, sleep};
use std::time::Duration;

struct Looper<Msg: Send + Sync> {
    sender: Sender<Msg>,
    handle: Option<thread::JoinHandle<()>>,
    stop_signal: Arc<(Mutex<bool>, Condvar)>
}

impl<Msg: Send + Sync + 'static> Looper<Msg> {
    fn new(process: fn(Msg) -> (), cleanup: fn() -> ()) -> Looper<Msg> {
        let (sender, receiver): (Sender<Msg>, Receiver<Msg>) = mpsc::channel();
        let stop_signal = Arc::new((Mutex::new(false), Condvar::new()));
        let stop_signal_clone = Arc::clone(&stop_signal);
        let receiver = Arc::new(Mutex::new(receiver));

        let handle = thread::spawn(move || {
            Looper::start_loop(receiver, process, cleanup, stop_signal_clone);
        });

        Looper {
            sender,
            handle: Some(handle),
            stop_signal
        }
    }

    pub fn send(&self, msg: Msg) -> Result<(), Box<dyn std::error::Error>>{
        Ok(self.sender.send(msg)?)
    }

    fn start_loop(receiver: Arc<Mutex<Receiver<Msg>>>, process: fn(Msg) -> (), cleanup: fn() -> (), stop_signal: Arc<(Mutex<bool>, Condvar)>)
    {
        loop {
            let msg = receiver.lock().unwrap().recv_timeout(Duration::from_millis(100));
            if let Ok(msg) = msg {
                process(msg);                
            } else {
                break;
            }

            {
                let result = stop_signal.1.wait_timeout_while(stop_signal.0.lock().unwrap(), Duration::from_millis(100), |stop| {*stop == true});
                let (end, error) = result.unwrap();
                if !error.timed_out() && *end {
                    break;
                }
            }
        }

        cleanup();
    }
}

impl<Msg: Send + Sync> Drop for Looper<Msg> {
    fn drop(&mut self) {
        *self.stop_signal.0.lock().unwrap() = true;
        self.stop_signal.1.notify_all();
        if let Some(handle) = self.handle.take() {
            handle.join().unwrap();
        }
    }
}

// Funzione di esempio per l'elaborazione dei messaggi
fn process_message<Msg: Sync + Debug>(msg: Msg) {
    println!("Processing Msg: {:?}", msg);
}

// Funzione di esempio per la pulizia
fn cleanup() {
    println!("Cleaning up...");
}

fn main() {
    let looper = Looper::new(process_message, cleanup);

    // Invia messaggi al looper
    let _ = looper.send("Message 1");
    let _ = looper.send("Message 2");

    // Il looper sarà automaticamente pulito quando esce dall'ambito o viene richiamata std::mem::drop(looper)
    // Attende un po' per vedere l'elaborazione
    sleep(Duration::from_secs(1));
}
