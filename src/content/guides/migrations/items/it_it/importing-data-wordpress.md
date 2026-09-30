Il nostro [Plugin WordPress](https://wordpress.org/plugins/fastcomments/) ha un potente meccanismo di importazione basato su UI. Dopo aver installato il plugin, ti guiderà nel collegare la tua installazione WordPress con FastComments e nel copiare i dati dei commenti esistenti.

**Questo avviene senza copiare o scaricare nulla manualmente.**

Il processo di migrazione ti verrà indicato tramite l'interfaccia utente durante la migrazione. La maggior parte delle migrazioni richiede solo un paio di minuti.

Il meccanismo è progettato per non sovraccaricare la tua installazione WordPress durante la migrazione.

Se stai spostando il tuo sito fuori da WordPress, puoi importare un'esportazione XML o CSV di WordPress invece di utilizzare il plugin. Vedi [Spostare i tuoi commenti su un nuovo sito](/guide-installation-wordpress.html#wordpress-moving-off-wordpress).

### CloudFlare & FireWalls

Affinché la configurazione automatica di WordPress funzioni, dobbiamo effettuare chiamate alla tua installazione WordPress. I firewall come Cloudflare possono bloccarci e causare il fallimento dell'integrazione. In tali casi, [possiamo fornirti](https://fastcomments.com/auth/my-account/help) un insieme di IP da inserire nella whitelist per l'integrazione.

### Data Ownership

Nel caso della nostra migrazione WordPress, tutti i nuovi o aggiornati dati dei commenti vengono sincronizzati automaticamente con la tua installazione WordPress in background. Questo significa che, mentre i commenti sono serviti da FastComments stesso per ridurre il carico sulla tua distribuzione WordPress, noi **anche** li salviamo nel tuo database come backup. Questo significa anche che, se desideri passare a un altro servizio, i tuoi dati sono già migrati e aggiornati.