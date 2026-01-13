### Real-Time Updates

Collab Chat utilizza connessioni WebSocket per sincronizzare tutte le conversazioni in tempo reale tra tutti gli utenti connessi. Quando qualcuno crea una nuova annotazione, aggiunge un commento o elimina una discussione, tutti gli altri utenti che visualizzano la stessa pagina vedono l'aggiornamento immediatamente senza dover aggiornare la pagina.

### How WebSocket Sync Works

Quando si inizializza Collab Chat, il widget stabilisce una connessione WebSocket con i server FastComments. Questa connessione rimane aperta per tutta la durata della sessione dell'utente e ascolta gli aggiornamenti relativi alla pagina corrente.

Il sistema WebSocket utilizza tre tipi di messaggi broadcast per Collab Chat. L'evento `new-text-chat` viene attivato quando qualcuno crea una nuova annotazione sulla pagina. L'evento `updated-text-chat` viene attivato quando qualcuno aggiorna una conversazione esistente. L'evento `deleted-text-chat` viene attivato quando qualcuno elimina un'annotazione.

### Broadcast ID System

Per evitare effetti di eco in cui gli utenti vedono le proprie azioni trasmesse nuovamente, ogni aggiornamento include un `broadcastId` univoco. Quando un utente crea o aggiorna un'annotazione, il suo client genera un UUID per quell'operazione. Quando il WebSocket trasmette l'aggiornamento a tutti i client, il client di origine ignora l'aggiornamento perché corrisponde al proprio `broadcastId`.

Questo garantisce un'interazione fluida in cui gli utenti vedono immediatamente le proprie modifiche nell'interfaccia senza dover attendere il giro di andata e ritorno attraverso il server, pur assicurando che tutti gli altri utenti ricevano l'aggiornamento.

### Live User Count

La barra superiore mostra il numero di utenti che stanno attualmente visualizzando la pagina. Questo conteggio si aggiorna in tempo reale man mano che gli utenti entrano e escono. Il conteggio degli utenti è fornito tramite la stessa connessione WebSocket e aumenta/diminuisce automaticamente in base agli eventi di connessione e disconnessione.

### Connection Resilience

Se la connessione WebSocket si interrompe a causa di problemi di rete o manutenzione del server, il widget tenta automaticamente di riconnettersi. Durante il periodo di riconnessione, gli utenti possono ancora interagire con le annotazioni esistenti, ma non vedranno aggiornamenti in tempo reale dagli altri utenti fino a quando la connessione non sarà ristabilita.

Una volta riconnesso, il widget si risincronizza per assicurarsi che non siano stati persi aggiornamenti. Questo avviene in modo trasparente senza richiedere interventi da parte dell'utente.

### Bandwidth Considerations

I messaggi WebSocket sono leggeri e contengono solo le informazioni essenziali necessarie per sincronizzare lo stato. La creazione di una nuova annotazione utilizza tipicamente meno di 1KB di larghezza di banda. Il sistema include anche un batching intelligente per ridurre la frequenza dei messaggi durante i periodi di alta attività.

Le tue metriche d'uso nella dashboard FastComments tracciano `pubSubMessageCount` e `pubSubBandwidth` in modo da poter monitorare l'attività di sincronizzazione in tempo reale sui tuoi siti.

### Cross-Tab Synchronization

Se un utente ha la stessa pagina aperta in più schede del browser, gli aggiornamenti in una scheda appaiono immediatamente nelle altre schede. Questo funziona tramite lo stesso meccanismo di sincronizzazione WebSocket e non richiede alcuna configurazione aggiuntiva.

### Security

I messaggi WebSocket vengono trasmessi su connessioni sicure (WSS) e includono la validazione del tenant per assicurare che gli utenti ricevano solo gli aggiornamenti delle conversazioni che sono autorizzati a visualizzare. Il server valida tutte le operazioni prima di trasmetterle per prevenire accessi o manipolazioni non autorizzate.