Il nostro [Plugin WordPress](https://wordpress.org/plugins/fastcomments/) dispone di un potente meccanismo di importazione basato sull’interfaccia utente. Dopo aver installato il plugin, ti guiderà nel collegare la tua installazione WordPress a FastComments e nel copiare i dati dei commenti esistenti.

**Questo avviene senza copiare o scaricare nulla manualmente.**

Il processo di migrazione ti verrà indicato tramite l’interfaccia utente durante la migrazione. La maggior parte delle migrazioni richiede solo pochi minuti.

Il meccanismo è progettato per non sovraccaricare eccessivamente la tua installazione WordPress durante la migrazione.

### CloudFlare & FireWalls

Perché la configurazione automatica di WordPress funzioni, dobbiamo effettuare chiamate alla tua installazione WordPress. I firewall come Cloudflare potrebbero bloccarci e far fallire l’integrazione. In tali casi, [possiamo fornirti](https://fastcomments.com/auth/my-account/help) un set di IP da inserire nella whitelist per l’integrazione.

### Data Ownership

Nel caso della nostra migrazione WordPress, tutti i nuovi o aggiornati dati dei commenti vengono sincronizzati automaticamente con la tua installazione WordPress in background. Ciò significa che, mentre i commenti sono serviti da FastComments stesso per ridurre il carico sulla tua distribuzione WordPress, noi **salviamo anche** i commenti nel tuo database come backup. Questo implica anche che, se desideri passare a un altro servizio, i tuoi dati sono già migrati e aggiornati.