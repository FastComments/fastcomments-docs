Se stai spostando il tuo sito da WordPress e vuoi FastComments sul nuovo sito, non hai bisogno del plugin WordPress. Esporta i tuoi commenti da
WordPress, quindi carica il file nella [pagina di importazione](https://fastcomments.com/auth/my-account/manage-data/import) nella dashboard di FastComments.

Supportiamo due formati di esportazione di WordPress.

### WordPress XML (Consigliato)

Questo è il file esportato dallo strumento integrato di WordPress, quindi non è necessario alcun plugin aggiuntivo.

1. Nel tuo amministratore WordPress, vai su `Tools -> Export`.
2. Seleziona `All content` e fai clic su `Download Export File`.
3. Nella [pagina di importazione](https://fastcomments.com/auth/my-account/manage-data/import) di FastComments, seleziona `WordPress (.xml)` e carica il file.

Ogni commento è associato all'URL del post su cui è stato lasciato, che è già presente nel file.

L'importazione conserva il nome dell'autore, l'email e il sito web, la data, il contenuto, la gerarchia delle risposte e se il commento è stato approvato. Gli avatar dei commentatori sono
importati da Gravatar. I voti non fanno parte di questo formato.

### WordPress CSV

Questo è il file proveniente dal [plugin WordPress Comments Import & Export di WebToffee](https://wordpress.org/plugins/comments-import-export-woocommerce/).

1. Installa il plugin nel tuo amministratore WordPress ed esporta i tuoi commenti come CSV.
2. Sostituisci ogni valore `comment_post_ID` con l'URL del post.
3. Nella [pagina di importazione](https://fastcomments.com/auth/my-account/manage-data/import) di FastComments, seleziona `WordPress (.csv)` e carica il file.

Ogni commento è associato alla colonna `comment_post_ID`. WordPress riempie questa colonna con l'ID del post, e il tuo nuovo sito non ha gli ID dei post di WordPress,
quindi il passaggio 2 lo sostituisce con l'URL.

L'importazione conserva il nome dell'autore, l'email e il sito web, la data, il contenuto, la gerarchia delle risposte e se il commento è stato approvato. Gli avatar dei commentatori sono
importati da Gravatar. Conserva anche il flag di spam di WordPress e i like e dislike di wpDiscuz quando il file li include.

### Associare i commenti alle tue nuove pagine

Se il tuo nuovo sito mantiene gli stessi URL del tuo sito WordPress, i commenti appariranno sulle pagine corrispondenti senza configurazioni aggiuntive.

Se il dominio cambia, esegui lo [strumento di migrazione del dominio](/guide-migrations.html#migrating-domains) dopo l'importazione. Se gli URL delle singole pagine cambiano, puoi
[migrare ogni pagina](/guide-migrations.html#migrating-pages) dal suo vecchio URL al nuovo.

Per migrazioni di pagine in blocco, come rimuovere il dominio dal valore che passi al campo [urlId](/guide-customizations-and-configuration.html#url-id) del widget dei commenti,
[apri un ticket di supporto](https://fastcomments.com/auth/my-account/help) e noi lo gestiremo per te.

### Prima di cambiare

Puoi eseguire l'importazione quante volte vuoi. Reimportare lo stesso file [non crea duplicati](/guide-migrations.html#importing-data), così puoi
importare una volta per testare il nuovo sito, poi importare di nuovo con i commenti più recenti proprio prima di effettuare il passaggio.

Per file di esportazione più grandi di 1 GB, [contatta il supporto](https://fastcomments.com/auth/my-account/help).

Per aggiungere FastComments al tuo nuovo sito, consulta la [guida all'installazione](/guide-installation.html).