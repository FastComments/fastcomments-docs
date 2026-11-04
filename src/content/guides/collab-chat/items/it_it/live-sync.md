### Aggiornamenti in tempo reale

Collab Chat utilizza connessioni WebSocket per sincronizzare tutte le conversazioni in tempo reale tra tutti gli utenti connessi. Quando qualcuno crea una nuova annotazione, aggiunge un commento o elimina una discussione, tutti gli altri utenti che stanno visualizzando la stessa pagina vedono l'aggiornamento immediatamente senza dover ricaricare.

### Come funziona la sincronizzazione WebSocket

Quando inizializzi Collab Chat, il widget stabilisce una connessione WebSocket ai server FastComments. Questa connessione rimane aperta per tutta la durata della sessione dell'utente e resta in ascolto degli aggiornamenti relativi alla pagina corrente.

Il sistema WebSocket utilizza tre tipi di messaggi broadcast per Collab Chat. L'evento `new-text-chat` viene attivato quando qualcuno crea una nuova annotazione sulla pagina. L'evento `updated-text-chat` viene attivato quando qualcuno aggiorna una conversazione esistente. L'evento `deleted-text-chat` viene attivato quando qualcuno elimina un'annotazione.

### Sistema di Broadcast ID

Per prevenire effetti eco in cui gli utenti vedono le proprie azioni ritrasmesse a loro stessi, ogni aggiornamento include un `broadcastId` univoco. Quando un utente crea o aggiorna un'annotazione, il suo client genera un UUID per quella operazione. Quando il WebSocket ritrasmette l'aggiornamento a tutti i client, il client di origine ignora l'aggiornamento perché corrisponde al proprio `broadcastId`.

Questo garantisce un'interazione fluida in cui gli utenti vedono le proprie modifiche immediatamente nell'interfaccia senza attendere il round-trip attraverso il server, assicurando al contempo che tutti gli altri utenti ricevano l'aggiornamento.

### Conteggio utenti in tempo reale

La barra superiore mostra il numero di utenti che stanno attualmente visualizzando la pagina. Questo conteggio si aggiorna in tempo reale man mano che gli utenti entrano ed escono. Il conteggio degli utenti viene fornito tramite la stessa connessione WebSocket e aumenta/diminuisce automaticamente in base agli eventi di connessione e disconnessione.

### Resilienza della connessione

Se la connessione WebSocket si interrompe a causa di problemi di rete o manutenzione del server, il widget tenta automaticamente di riconnettersi. Durante il periodo di riconnessione, gli utenti possono comunque interagire con le annotazioni esistenti, ma non vedranno gli aggiornamenti in tempo reale degli altri utenti finché la connessione non sarà ristabilita.

Una volta riconnesso, il widget si risincronizza per assicurarsi che nessun aggiornamento sia stato perso. Questo avviene in modo trasparente senza richiedere l'intervento dell'utente.

### Considerazioni sulla larghezza di banda

I messaggi WebSocket sono leggeri e contengono solo le informazioni essenziali necessarie per sincronizzare lo stato. La creazione di una nuova annotazione utilizza tipicamente meno di 1KB di larghezza di banda. Il sistema include anche un batching intelligente per ridurre la frequenza dei messaggi durante i periodi di alta attività.

Le tue metriche di utilizzo nella dashboard di FastComments tracciano `pubSubMessageCount` e `pubSubBandwidth` così puoi monitorare l'attività di sincronizzazione in tempo reale dei tuoi siti.

### Sincronizzazione tra schede

Se un utente ha la stessa pagina aperta in più schede del browser, gli aggiornamenti in una scheda appaiono immediatamente nelle altre schede. Questo funziona attraverso lo stesso meccanismo di sincronizzazione WebSocket e non richiede alcuna configurazione aggiuntiva.

### Sicurezza

I messaggi WebSocket vengono trasmessi su connessioni sicure (WSS) e includono la validazione del tenant per garantire che gli utenti ricevano solo gli aggiornamenti delle conversazioni che sono autorizzati a vedere. Il server valida tutte le operazioni prima di ritrasmetterle per prevenire accessi o manipolazioni non autorizzati.