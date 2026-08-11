Nel caso in cui sia necessario spostare i dati, FastComments offre uno strumento self‑service per spostare i commenti tra pagine e articoli.

Ecco come appare il modulo di copia dei commenti:

[app-screenshot-start url='/auth/my-account/manage-data/copy-comments'; selector = '.account-block'; alt='Modulo di copia commenti con il campo ID URL di origine e i campi ID URL di destinazione e URL'; title='Il modulo di copia commenti' app-screenshot-end]

### Compilare i campi "Da"

Per decidere da dove spostare i commenti, è sufficiente conoscere l'`URL ID` di origine.

Se non stai passando un valore per `urlId` nella configurazione del widget dei commenti, allora questo sarà una versione "pulita" dell'URL della pagina.

Puoi vedere quali valori hanno i tuoi commenti per `URL ID` esportandoli.

### Compilare i campi "A"

Per decidere dove spostare i commenti, dobbiamo conoscere l'`URL ID` e l'`URL` di destinazione.

L'`URL ID` sarà il contenitore in cui il commento verrà inserito. Il campo `URL` è usato in modo da poter navigare direttamente al commento da email e strumenti di moderazione.

#### WordPress

Se stai usando WordPress, ad esempio inseriresti gli ID degli articoli nei campi `URL ID` Da/A nello strumento di migrazione, anziché un URL.