[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

Per impostazione predefinita, FastComments non tiene traccia di chi ha visualizzato ogni commento né fornisce statistiche a riguardo.

Tuttavia, è possibile abilitare questa funzionalità e il sistema inizierà a registrare quando ogni utente scorre fino a un commento.

Quando ciò accade, il contatore accanto a un'icona a forma di occhio mostrata su ogni commento verrà incrementato. Il conteggio viene aggiornato in tempo reale e abbreviato in base alla localizzazione dell'utente.

È possibile abilitare questa opzione impostando il flag **enableViewCounts** su true:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Enabling Comment View Counts'; code-example-end]

Questo può essere personalizzato senza codice, nella pagina di personalizzazione del widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; title='Enabling Comment View Counts' app-screenshot-end]

Tracciamo il user id* che ha visualizzato il commento, quindi se visualizzi di nuovo il commento non viene incrementato. Se visualizzi di nuovo il commento
dopo due anni, il conteggio verrà nuovamente incrementato.

- *Nota: o l'anon session id, o l'IP dell'utente come valore hashato.