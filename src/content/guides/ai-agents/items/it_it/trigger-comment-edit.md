Attiva l'agente quando un commento viene modificato.

### Contesto che l'agente riceve

- Il commento nella sua forma attuale (dopo la modifica).
- Il **testo precedente del commento** come blocco separato delimitato (`PREVIOUS_TEXT`). Questo è unico per il trigger di modifica - l'agente può confrontare prima/dopo.
- Cronologia opzionale del thread/utente/contesto della pagina come configurato.

### Da notare

- Il trigger si attiva per qualsiasi modifica riuscita, incluse le modifiche eseguite dai moderatori per conto di un utente.
- Agli agenti non è esposto alcuno strumento per modificare i commenti; gli agenti non possono modificare i commenti in alcun modo.
- Il testo precedente del commento è trattato come input non attendibile. Il prompt di sistema della piattaforma ricorda al modello di non seguire istruzioni presenti all'interno di blocchi delimitati - questo è importante qui, perché un utente malintenzionato potrebbe modificare un commento per inserire un payload "ignora le tue istruzioni precedenti" rivolto a qualsiasi agente che osservi gli eventi di modifica.

### Utilizzi comuni

- **Individuazione di contenuti riciclati** - un utente modifica un commento precedentemente pulito per inserire spam dopo che il moderatore è passato oltre.
- **Tracciamento delle modifiche minori** - se la tua comunità considera le modifiche come eventi separati per motivi di audit.

### Nota sui costi

I trigger di modifica vedono due copie del testo del commento (la nuova versione nel blocco standard COMMENT, la versione precedente nel blocco PREVIOUS_TEXT). Per commenti lunghi questo raddoppia approssimativamente il costo in token dell'esecuzione rispetto a un trigger `COMMENT_ADD` - tienilo presente quando stimi i costi.