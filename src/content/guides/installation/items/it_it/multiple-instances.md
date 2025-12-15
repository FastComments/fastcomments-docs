Ogni istanza del widget dei commenti è isolata. Per questo motivo, FastComments supporta nativamente più di un'istanza per pagina, o più istanze che puntano allo stesso thread di chat.

Nel caso della libreria VanillaJS, ad esempio, devi semplicemente collegare il widget dei commenti a diversi nodi DOM. Se vuoi semplicemente aggiornare il thread corrente sulla pagina, vedi [Cambiare thread di commenti senza ricaricare la pagina](guide-customizations-and-configuration.html#switching-comment-threads);

### Sincronizzazione dello stato di autenticazione tra più istanze

Esaminiamo l'esempio di un'applicazione single-page personalizzata che è una lista di domande frequenti con il proprio thread di commenti.

In questo caso, abbiamo più istanze di FastComments nel DOM contemporaneamente.

Questo va bene, ma pone alcune sfide per l'esperienza utente.

Considera questo flusso:

1. L'utente visita la pagina con una lista di domande, ognuna con il proprio widget dei commenti.
2. L'utente inserisce il proprio nome utente ed email e lascia una domanda in uno dei thread.
3. Vede un altro elemento FAQ su cui ha una domanda.
4. Va a commentare di nuovo. Deve inserire nuovamente email e nome utente?

In questo caso, FastComments gestisce la sincronizzazione dello stato di autenticazione tra le istanze del widget per te. Nel quarto passaggio, l'utente sarà già temporaneamente autenticato poiché ha inserito nome utente ed email sulla stessa pagina.
