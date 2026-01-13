Nel caso sia necessario spostare dei dati, FastComments mette a disposizione uno strumento self-service per spostare i commenti
tra pagine e articoli.

Ecco l'aspetto del modulo di copia dei commenti:

[app-screenshot-start url='/auth/my-account/manage-data/copy-comments'; selector = '.account-block'; title='The Copy Comment Form' app-screenshot-end]

### Compilazione dei campi "From"

Per decidere da dove spostare i commenti, abbiamo semplicemente bisogno di conoscere il `URL ID` di origine.

Se non stai passando un valore per `urlId` nella configurazione del widget dei commenti, allora questa sarà una versione "pulita" dell'URL della pagina.

### Compilazione dei campi "To"

Per decidere dove spostare i commenti, abbiamo bisogno di conoscere il target `URL ID` e l'`URL`.

Il `URL ID` sarà il bucket in cui viene inserito il commento. Il campo `URL` viene usato in modo da poter navigare direttamente
al commento da email e dagli strumenti di moderazione.

#### WordPress

Se usi WordPress, ad esempio inseriresti gli ID degli articoli nei campi To/From `URL ID` nello strumento di migrazione,
piuttosto che un URL.