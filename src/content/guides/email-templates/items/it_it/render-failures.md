Poiché i template email supportano variabili, e logica, è possibile creare template
che non riescono a essere renderizzati, o che a volte falliscono nel rendering.

Questo può essere molto frustrante da diagnosticare e fare il debug, specialmente se si tratta di un problema intermittente, o
se si verifica solo quando i dati hanno un certo aspetto.

Per aiutare, FastComments Email Templates ha un paio di funzionalità:

1. Se il template non riesce ad essere antevisualizzato, non può essere salvato. Verrà mostrato un messaggio di errore.
2. I fallimenti nel rendering dei template vengono tracciati e riportati nell'interfaccia di amministrazione.

Il secondo punto descrive i fallimenti di rendering che avvengono in produzione. Cioè, crei un template che in anteprima
va bene - ma in seguito fallisce per qualche motivo. Per esempio, se abbiamo questo nel nostro template:

    <% if (comment.commenterEmail.includes('test') { %>

Questo può fallire alcune volte se abbiamo i commenti anonimi abilitati, poiché l'email non sarà sempre
disponibile. Quindi come scopriamo questo?

La risposta è che gli errori vengono resi visibili in due punti. Primo, la lista dei template stessa
mostra un conteggio degli errori di rendering per ogni template.

Inoltre, quando si visualizza un template possiamo vedere un conteggio, per errore, del numero di volte in cui il template
ha fallito nel rendering.

Un pulsante di reset è posizionato accanto a ciascun errore e al suo conteggio, in modo da poter azzerare il contatore
dopo aver risolto il problema.