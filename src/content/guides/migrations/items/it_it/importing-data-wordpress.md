Il nostro [WordPress Plugin](https://wordpress.org/plugins/fastcomments/) dispone di un potente meccanismo di importazione basato sull'interfaccia utente. Una volta installato il plugin, ti guiderà nel collegare la tua installazione WordPress a FastComments e nel copiare i tuoi dati dei commenti esistenti.

**Questo avviene senza copiare o scaricare nulla manualmente.**

Il processo di migrazione ti verrà indicato tramite l'interfaccia utente durante la migrazione. La maggior parte delle migrazioni richiede solo pochi minuti.

Il meccanismo è progettato per non sovraccaricare la tua installazione WordPress durante la migrazione.

Se stai spostando il tuo sito fuori da WordPress, puoi importare un'esportazione XML o CSV di WordPress invece di utilizzare il plugin. Vedi [Spostare i commenti su un nuovo sito](/guide-installation-wordpress.html#wordpress-moving-off-wordpress).

### CloudFlare e firewall

Affinché la configurazione automatica di WordPress funzioni, dobbiamo effettuare chiamate alla tua installazione WordPress. I firewall come Cloudflare possono bloccarci e far fallire l'integrazione. In tali casi, [possiamo fornirti](https://fastcomments.com/auth/my-account/help) un insieme di IP da inserire nella whitelist per l'integrazione.

### Proprietà dei dati

Nel caso della nostra migrazione WordPress, tutti i nuovi o aggiornati dati dei commenti vengono sincronizzati automaticamente con la tua installazione WordPress in background. Questo significa che, mentre i commenti sono serviti da FastComments stesso per ridurre il carico sulla tua distribuzione WordPress, noi **anche** li salviamo nel tuo database come backup. Ciò significa anche che, se desideri passare a un altro servizio, i tuoi dati sono già migrati e aggiornati.