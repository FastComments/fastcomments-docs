Tutte le modifiche all'oggetto Comment nel sistema generano un evento che finisce in una coda.

L'evento webhook iniziale viene solitamente inviato entro sei secondi dall'occorrenza della sorgente dell'evento.

Puoi monitorare questa coda nell'amministrazione Webhooks nel caso in cui la tua API vada offline.

Se una richiesta alla tua API fallisce, la rimetteremo in coda secondo una pianificazione.

Quella pianificazione è `1 Minute * the retry count`. Se la chiamata fallisce una volta, riproverà in
un minuto. Se fallisce due volte, allora aspetterà due minuti, e così via. Questo serve a evitare che
sovraccarichiamo la tua API se stai andando offline per motivi legati al carico.

I Webhooks possono essere annullati dalla [pagina dei log](https://fastcomments.com/auth/my-account/manage-data/webhooks/logs).