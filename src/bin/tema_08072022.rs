// Domanda 1: Si definiscano i concetti di Dangling Pointer, Memory Leakage e Wild Pointer, facendo esempi
// concreti, usando dello pseudocodice, che possono generare questi fenomeni.
//
// Data l'esistenza di un puntatore in un programma ad una locazione della memoria ram, è possibile
// che si verifichino errori impredicibili dovuti al modo in cui questi puntatori vengono usati.
// I fenomeni di cui sopra sono tipologie di questi errori, un dangling pointer è un puntatore
// pendente ovvero un riferimento all'indirizzo di memoria di un dato che è stato deallocato,
// questo errore in linguaggi diversi da rust dove l'utilizzo dei puntatori nativi è permesso come C, C++
// è difficile da individuare e si verifica in fase di esecuzione durante l'utilizzo del programma.
// Il fenomeno di memory leakage è costituito invece dalla mancata deallocazione di una zona di
// memoria allocata e individuata dal puntatore e della conseguente riduzione della memoria
// disponbibile fino ad esaurimento se si continua ad allocare memoria senza rilasciarla fino al
// termine del programma, questo tipo di errore diventa dannoso quando un programma come un server
// viene eseguito senza che si voglia mai interromperne l'esecuzione.
// Un wild pointer è un fenomeno in cui tramite operazioni aritmetiche si cambia l'indirizzo a cui
// il puntatore è connesso comportando che tutte le operazioni successive si riferiscono ad un dato
// non conosciuto e/o non allocato dallo stesso processo, portando cosi' a comportamenti
// impredicibili.
//
// Esempio di dangling pointer in C:
//
// int main(int* argc, char** argv) {
//      int* p = (int*)malloc(5*sizeof(int));
//      free(p);
//      //Utilizzo di un puntatore deallocato -> dangling pointer
//      printf("%d",p[0]);
//      return 0;
// } 
//
// Esempio di memory leakage in C: 
//
// int main(int* argc, char** argv) {
//      while 0 {
//          int* p = (int*)malloc(5*sizeof(int));
//          //All' allocazione n-esima dove in questo esempio si allocano 5 byte ogni giro di while e n sarà MEMORIA RAM / 5B
//          //Si esaurirà la memoria per memory leakage
//
//      }
//      return 0;
// }
//
// Esempio di wild pointer in C:
//
// int main(int* argc, char** argv) {
//      //il vettore p viene creato
//      int* p = (int*)malloc(5*sizeof(int));
//      //sposto l'indirizzo del puntatore ad un nuovo indirizzo maggiore (NB: Non posso sapere il
//      //nuovo indirizzo fino a che il programma non viene eseguito)
//      *p += *p/2;
//      //utilizzo il nuovo puntatore -> Comportamento impredicibile o errore per accesso illegale
//      //alla memoria
//      printf("%d",p[0]);
//      return 0;
// }
//
// Domanda 2: In relazione al concetto di Atomic, si definisca cosa esso mira a garantire, come tale garanzia
// possa essere fornite a livello architetturale, e quali siano i suoi limiti.
//
// Il concetto di Atomic prende in prestito il suo significato dal significato comune di atomico
// cioè indivisibile. Un dato atomico è un dato su cui le operazioni logiche e matematiche sono
// indivisibili ovvero nella memoria del computer non presentano stati intermedi nel loro calcolo.
// Questa tipologia di dato viene usata all'interno di contesti multithreaded, in quanto data
// l'imprevedibilità della velocità dei thread allocati sul sistema dell'utente che usa un
// programma multithreaded (ma anche in fase di programmazione sul proprio dispositivo) i thread
// possono fare accesso ai dati in ordine non prevedibile, generalizzando le operazioni sui dati
// sono letture o scritture e se un dato nella scrittura viene variato assumendo stati intermedi
// per poter poi essere scritto nella sua forma finale (risultato del operazione di scrittura) i
// thread che leggono potrebbero usare un valore non valido. Per questo assicurarsi che i thread
// usino un dato di tipo atomico limita la tipologia di errori possibili in fase di esecuzione.
// Inoltre quando possibile implementare la logica di un programma con un dato di tipo atomico
// garantisce una performance migliore rispetto all'utilizzo di meccanismi di sincronizzazione.
// Sebbene però i dati atomici siano un alternativa migliore degli altre tipologie di
// sincronizzazione questi non possono essere sempre implementati. Dati che racchiudono l'utilizzo di
// riferimenti alla memoria ad esempio, non possono avere una forma atomica poichè implicitamente
// necessitano di operazioni sui dati a cui a loro volta puntano.
//
// Domanda 3: All'interno di un programma è definita la seguente struttura dati
// struct Bucket {
//  data: Vec<i32>,
//  threshold: Option<i32>
//  }
// Usando il debugger si è determinato che, per una istanza di Bucket, essa è memorizzata
// all'indirizzo 0x00006000014ed2c0.
// Osservando la memoria presente a tale indirizzo, viene mostrato il seguente contenuto (per
// blocchi di 32bit):
// 308a6e01 00600000 03000000 00000000 03000000 00000000 01000000 0a000000
// Cosa è possibile dedurre relativamente ai valori contenuti dei vari campi della singola istanza?
//
// Conoscendo il tipo dei dati della struct Bucket è possibile dedurre che:
// I primi 4 byte conterranno l'indirizzo della memoria a cui il vettore comincia, i secondi 4 byte la sua
// dimensione, a seguire gli ultimi 4 conterranno il dato chiamato "threshold". Se si guarda la
// memoria all'indirizzo dato quindi si evince che:
// 1. L'indirizzo nella memoria ram del vettore è: 0x308a6e01
// 2. La dimensione del vettore è: 0x00600000
// 3. Il valore del campo threshold è: 0x03000000 -> dato che si tratta di un numero con segno si
//    può calcolarne il valore sapendo già a prima vista che si tratta di un numero positivo.
//
// Domanda 4: All'interno di un programma è necessario garantire che non vengano eseguite
// CONTEMPORANEAMENTE più di N invocazioni di operazioni potenzialmente lente.
// A questo scopo, è stata definita la struttura dati ExecutionLimiter che viene inizializzata con il
// valore N del limite. Tale struttura è thread-safe e offre solo il metodo pubblico generico execute( f
// ), che accetta come unico parametro una funzione f, priva di parametri che ritorna il tipo generico
// R. Il metodo execute(...) ha, come tipo di ritorno, lo stesso tipo R restituito da f ed ha il compito di
// mantere il conteggio di quante invocazioni sono in corso. Se tale numero è già pari al valore N
// definito all'atto della costruzione della struttura dati, attende, senza provocare consumo di CPU,
// che scenda sotto soglia, dopodiché invoca la funzione f ricevuta come parametro e ne restituisce il
// valore. Poiché l'esecuzione della funzione f potrebbe fallire, in tale caso, si preveda di
// decrementare il conteggio correttamente.
// Si implementi, usando i linguaggi Rust o C++, tale struttura dati, garantendo tutte le funzionalità
// richieste.
//
//
use std::sync::{ Mutex, Condvar, Arc };
use std::thread::spawn;
use rand::{Rng, thread_rng};

struct ExecutionLimiter {
    lock: Mutex<usize>,
    condvar: Condvar,
    n: usize
}

impl ExecutionLimiter {
    
    pub fn new(n: usize) -> ExecutionLimiter {
        ExecutionLimiter {
            lock: Mutex::new(0),
            condvar: Condvar::new(),
            n
        }
    }

    pub fn execute<R: Sized>(&mut self, f: impl Fn() -> Result<R, ()>) -> Option<R> {
        let mut current_n_threads = self.condvar
        .wait_while(
            self.lock.lock().unwrap(),
            |n_threads_active| *n_threads_active > self.n
        ).unwrap();
        *current_n_threads += 1;
        if let Ok(result) = f() {
            *current_n_threads -= 1;
            return Some(result);
        } else {
            *current_n_threads -= 1;
            return None;
        }
    }

}

pub fn main() {
    let limiter = Arc::new(Mutex::new(ExecutionLimiter::new(3)));
    let function = || {
        let mut rng = thread_rng();
        let random = rng.gen_range(0..4);
        if random > 0 {
            return Ok(random);
        } else {
            return Err(());
        }
    };
    let mut handles = vec![];

    for _ in 0..5 {
        let mut_clone = Arc::clone(&limiter);
        let handle = spawn(move || {
            let result = mut_clone.lock().unwrap().execute(function);
            if let Some(result) = result {
                println!("{}",result);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
