[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

Per impostazione predefinita, FastComments non traccia chi ha visualizzato ogni commento né fornisce statistiche a riguardo.

Tuttavia, possiamo abilitare questa funzionalità e il sistema inizierà a tracciare quando ogni utente scorre verso un commento.

Quando ciò accade, un contatore accanto a un'icona a forma di occhio mostrata su ogni commento verrà incrementato. Il conteggio è aggiornato in tempo reale e abbreviato in base alla lingua dell'utente.

Possiamo abilitare questa opzione impostando il flag **enableViewCounts** su true:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Abilitare il conteggio delle visualizzazioni dei commenti'; code-example-end]

Questo può essere personalizzato senza codice, nella pagina di personalizzazione del widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; alt='Pagina di personalizzazione del widget con la casella dei conteggi visualizzazioni selezionata, così ogni commento mostra un\'icona a forma di occhio e il conteggio'; title='Abilitare il conteggio delle visualizzazioni dei commenti' app-screenshot-end]

Tracciamo l'ID utente* che ha visualizzato il commento, in modo che se visualizzi nuovamente il commento non venga incrementato. Se visualizzi nuovamente il commento
dopo due anni, il conteggio aumenterà di più.

- *Nota: oppure l'ID della sessione anonima, oppure l'IP dell'utente come valore hash.