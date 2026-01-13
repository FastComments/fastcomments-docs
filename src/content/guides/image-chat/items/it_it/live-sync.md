### Aggiornamenti in tempo reale

Image Chat usa connessioni WebSocket per sincronizzare tutte le conversazioni in tempo reale tra tutti gli utenti connessi. Quando qualcuno crea un nuovo marcatore, aggiunge un commento o elimina una discussione, tutti gli altri utenti che visualizzano la stessa immagine vedono l'aggiornamento immediatamente senza ricaricare.

### Come funziona la sincronizzazione via WebSocket

Quando inizializzi Image Chat, il widget stabilisce una connessione WebSocket ai server FastComments. Questa connessione rimane aperta per tutta la durata della sessione dell'utente e ascolta gli aggiornamenti relativi all'immagine corrente.

Il sistema WebSocket utilizza tre tipi di messaggi broadcast per Image Chat. L'evento `new-image-chat` viene attivato quando qualcuno crea un nuovo marcatore sull'immagine. L'evento `image-chat-updated` viene attivato quando qualcuno aggiorna una conversazione esistente. L'evento `deleted-image-chat` viene attivato quando qualcuno elimina un marcatore.

### Sistema di Broadcast ID

Per evitare effetti di eco in cui gli utenti vedono le proprie azioni ritrasmesse, ogni aggiornamento include un `broadcastId` univoco. Quando un utente crea o aggiorna un marcatore, il suo client genera un UUID per quell'operazione. Quando il WebSocket ritrasmette l'aggiornamento a tutti i client, il client di origine ignora l'aggiornamento perché corrisponde al proprio `broadcastId`.

Questo assicura un'interazione fluida in cui gli utenti vedono immediatamente le proprie modifiche nell'interfaccia senza attendere il round-trip attraverso il server, pur garantendo che tutti gli altri utenti ricevano l'aggiornamento.

### Resilienza della connessione

Se la connessione WebSocket cade a causa di problemi di rete o manutenzione del server, il widget tenta automaticamente di riconnettersi. Durante il periodo di riconnessione, gli utenti possono comunque interagire con i marcatori esistenti, ma non vedranno aggiornamenti in tempo reale da altri utenti fino a quando la connessione non sarà ristabilita.

Una volta riconnesso, il widget risincronizza per assicurarsi che non siano stati persi aggiornamenti. Questo avviene in modo trasparente senza richiedere l'intervento dell'utente.

### Considerazioni sulla larghezza di banda

I messaggi WebSocket sono leggeri e contengono solo le informazioni essenziali necessarie per sincronizzare lo stato. La creazione di un nuovo marcatore solitamente usa meno di 1KB di larghezza di banda. Il sistema include anche un batching intelligente per ridurre la frequenza dei messaggi durante i periodi di alta attività.

Le metriche di utilizzo nella dashboard di FastComments tracciano `pubSubMessageCount` e `pubSubBandwidth` in modo da poter monitorare l'attività di sincronizzazione in tempo reale sui tuoi siti.

### Sincronizzazione tra schede

Se un utente ha la stessa pagina aperta in più schede del browser, gli aggiornamenti in una scheda appaiono immediatamente nelle altre. Questo funziona tramite lo stesso meccanismo di sincronizzazione WebSocket e non richiede alcuna configurazione aggiuntiva.

Gli utenti possono avere il tuo sito aperto su più dispositivi contemporaneamente e tutti resteranno sincronizzati. Un marcatore creato su un computer desktop appare istantaneamente sul tablet dell'utente se entrambi i dispositivi stanno visualizzando la stessa immagine.

### Sicurezza

I messaggi WebSocket vengono trasmessi su connessioni sicure (WSS) e includono la validazione del tenant per assicurare che gli utenti ricevano solo gli aggiornamenti delle conversazioni per le quali sono autorizzati. Il server valida tutte le operazioni prima di trasmetterle per prevenire accessi o manipolazioni non autorizzate.

### Comportamento offline

Quando gli utenti sono completamente offline, possono ancora visualizzare i marcatori esistenti ma non possono crearne di nuovi né vedere gli aggiornamenti degli altri. Il widget rileva lo stato offline e visualizza messaggi appropriati.

Se un utente tenta di creare un marcatore mentre è offline e poi torna online, l'operazione fallirà anziché essere messa in coda, garantendo la coerenza dei dati. Gli utenti devono riprovare l'operazione una volta ripristinata la connessione.

### Impatto sulle prestazioni

La connessione WebSocket ha un impatto minimo sulle prestazioni. La connessione rimane inattiva quando non si verificano aggiornamenti e processa messaggi solo quando c'è attività. Su un'immagine tipica con attività moderata di marcatori, il WebSocket usa meno CPU rispetto al rendering dell'immagine stessa.

Per pagine con centinaia di utenti simultanei e alta attività di creazione marcatori, il sistema scala orizzontalmente per mantenere le prestazioni senza influire sulle singole connessioni client.

### Casi d'uso collaborativi

La sincronizzazione in tempo reale rende Image Chat particolarmente potente per flussi di lavoro collaborativi. I team di design possono rivedere insieme i mockup con tutti che vedono le posizioni dei marcatori in tempo reale. I team di supporto clienti possono annotare collaborativamente screenshot per identificare problemi. I gruppi educativi possono discutere diagrammi con tutti i partecipanti che vedono i marcatori degli altri mentre vengono creati.

Il feedback immediato crea un'esperienza collaborativa più coinvolgente e produttiva rispetto ai sistemi di commento tradizionali in cui gli utenti devono aggiornare la pagina per vedere gli aggiornamenti.

---