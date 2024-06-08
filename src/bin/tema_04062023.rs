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
use std::rc::Rc;
use std::borrow::{Borrow, BorrowMut};

struct QueueItem {
    value: usize,
    next: Option<Rc<QueueItem>>,
}

fn rc_example() {
    let mut item = Rc::new(QueueItem {
        value: 5,
        next: None
    });
    let item2 = Rc::new(QueueItem {
        value: 10,
        next: Some(Rc::clone(&item))
    });
    // Make the first node point to the second node
    if let Some(item_ref) = item.borrow_mut().next.as_ref() {
        // Use borrow_mut() to get a mutable reference to the inner Node inside the RefCell
        // Then, access the next field
        // Note: This won't panic as we're only borrowing mutably inside this scope
        if let Some(inner) = item_ref.borrow().as_ref() {
            let mut inner_mut = inner.borrow_mut();
            inner_mut.next = Some(Rc::clone(&item2));
        }
    } else {
        println!("Failed to get mutable reference to item");
    }
}

pub fn main() {
    rc_example();
}
