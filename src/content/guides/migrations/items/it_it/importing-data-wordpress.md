---
Il nostro [WordPress Plugin](https://wordpress.org/plugins/fastcomments/) ha un potente meccanismo di importazione basato su interfaccia grafica. Al momento dell'installazione del plugin,
ti guiderà attraverso il collegamento della tua installazione WordPress con FastComments e la copia dei dati dei commenti esistenti.

**Questo viene fatto senza copiare o scaricare nulla manualmente.**

Il processo di migrazione ti verrà indicato tramite l'interfaccia utente durante la migrazione. La maggior parte delle migrazioni richiede solo un paio di minuti.

Il meccanismo è progettato per non mettere un carico eccessivo sulla tua installazione WordPress durante la migrazione.

### CloudFlare & FireWalls

Perché la configurazione automatizzata di WordPress funzioni, dobbiamo effettuare chiamate alla tua installazione WordPress.
Firewall come Cloudflare potrebbero bloccarci e far fallire l'integrazione. In tali casi, [possiamo fornirti](https://fastcomments.com/auth/my-account/help) un insieme di IP da mettere in whitelist per l'integrazione.

### Data Ownership

Nel caso della nostra migrazione da WordPress, qualsiasi dato di commento nuovo o aggiornato viene automaticamente sincronizzato nuovamente con la tua installazione WordPress
in background. Ciò significa che, mentre i commenti sono serviti da FastComments stesso per alleggerire il carico sulla tua installazione WordPress,
noi li salviamo **anche** nel tuo database come backup. Questo significa inoltre che, se desideri passare via da FastComments, i tuoi dati sono
già migrati e aggiornati.
---