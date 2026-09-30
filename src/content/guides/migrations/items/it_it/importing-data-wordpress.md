---
Il nostro [WordPress Plugin](https://wordpress.org/plugins/fastcomments/) ha un potente meccanismo di importazione basato sull'interfaccia utente. Dopo aver installato il plugin, ti guiderà nel collegare la tua installazione WordPress a FastComments e nel copiare i tuoi dati dei commenti esistenti.

**Questo avviene senza copiare o scaricare nulla manualmente.**

Il processo di migrazione ti verrà indicato tramite l'interfaccia utente durante la migrazione. La maggior parte delle migrazioni richiede solo pochi minuti.

Il meccanismo è progettato per non sovraccaricare la tua installazione WordPress durante la migrazione.

Se stai spostando il tuo sito fuori da WordPress, puoi importare un'esportazione XML o CSV di WordPress invece di usare il plugin. Vedi [Spostare i tuoi commenti su un nuovo sito](/guide-installation-wordpress.html#wordpress-moving-off-wordpress).

### CloudFlare e firewall

Affinché la configurazione automatica di WordPress funzioni, dobbiamo effettuare chiamate alla tua installazione WordPress. I firewall come Cloudflare possono bloccarci e far fallire l'integrazione. In questi casi, [possiamo fornirti](https://fastcomments.com/auth/my-account/help) un insieme di IP da inserire nella whitelist per l'integrazione.

### Proprietà dei dati

Nel caso della nostra migrazione WordPress, tutti i nuovi dati dei commenti o quelli aggiornati vengono sincronizzati automaticamente con la tua installazione WordPress in background. Ciò significa che, mentre i commenti sono serviti da FastComments stesso per ridurre il carico sulla tua installazione WordPress, noi **salviamo anche** i commenti nel tuo database come backup. Questo significa anche che, se desideri passare a un altro servizio rispetto a FastComments, i tuoi dati sono già migrati e aggiornati.