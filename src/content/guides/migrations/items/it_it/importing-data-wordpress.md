Il nostro [plugin WordPress](https://wordpress.org/plugins/fastcomments/) dispone di un potente meccanismo di importazione basato su interfaccia grafica. Dopo l'installazione del plugin,
ti guideremo attraverso il collegamento della tua installazione WordPress con FastComments e la copia dei dati dei commenti esistenti.

**Tutto questo avviene senza dover copiare o scaricare nulla manualmente.**

Il processo di migrazione ti verrà indicato tramite l'interfaccia utente durante la migrazione. La maggior parte delle migrazioni richiede solo un paio di minuti.

Il meccanismo è progettato per non sovraccaricare eccessivamente la tua installazione WordPress durante la migrazione.

Se stai spostando il tuo sito fuori da WordPress, puoi importare un'esportazione XML o CSV di WordPress invece di utilizzare il plugin. Consulta
[Spostare i tuoi commenti su un nuovo sito](/guide-installation-wordpress.html#wordpress-moving-off-wordpress).

### CloudFlare e Firewall

Affinché la configurazione automatica di WordPress funzioni, dobbiamo effettuare chiamate alla tua installazione WordPress.
Firewall come Cloudflare potrebbero bloccarci e causare il fallimento dell'integrazione. In questi casi, [possiamo fornirti](https://fastcomments.com/auth/my-account/help) un elenco di IP da inserire nella whitelist per l'integrazione.

### Proprietà dei dati

Nel caso della nostra migrazione WordPress, tutti i nuovi dati dei commenti o quelli aggiornati vengono sincronizzati automaticamente con la tua installazione WordPress
in background. Ciò significa che, mentre i commenti sono serviti da FastComments stesso per alleggerire il carico sulla tua distribuzione WordPress,
noi li salviamo **anche** nel tuo database come backup. Questo significa anche che, se desideri passare a un servizio diverso da FastComments, i tuoi dati sono
già migrati e aggiornati.