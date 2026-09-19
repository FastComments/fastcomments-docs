[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

Per impostazione predefinita, FastComments non traccia chi ha visualizzato ogni commento né fornisce statistiche a riguardo.

Tuttavia, possiamo abilitare questa funzionalità e il sistema inizierà a tracciare quando ogni utente scorre verso un commento.

Quando ciò accade, un contatore accanto a un'icona a forma di occhio mostrata su ogni commento verrà incrementato. Il contatore viene aggiornato in tempo reale e abbreviato in base alla lingua dell'utente.

Possiamo abilitare questa opzione impostando il flag **enableViewCounts** su true:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Enabling Comment View Counts'; code-example-end]

Questo può essere personalizzato senza codice, nella pagina di personalizzazione del widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; alt='Pagina di personalizzazione del widget con la casella di controllo dei conteggi visualizzazioni selezionata, così ogni commento mostra l\'icona a forma di occhio e il conteggio'; title='Abilitazione dei conteggi visualizzazioni dei commenti' app-screenshot-end]

Tracciamo l'ID utente* che ha visualizzato il commento per una settimana, in modo che se visualizzi nuovamente il commento entro quella settimana il contatore non aumenti. Se visualizzi nuovamente il commento dopo che la settimana è trascorsa, il contatore aumenterà di nuovo.

- *Nota: oppure l'ID della sessione anonima, oppure l'IP dell'utente come valore hash.