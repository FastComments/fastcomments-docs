I webhook degli agenti ripetono i tentativi in caso di errore. La consegna è **fire-and-forget dal punto di vista dell'agente** - una consegna fallita non blocca l'esecuzione dell'agente né ripristina alcuna azione - e una coda + cron gestisce i retry in modo asincrono.

### Modello della coda

Ogni evento viene messo in coda **una volta per webhook corrispondente**. Quindi, se hai tre webhook iscritti a `trigger.succeeded` per un dato agente + dominio, la piattaforma mette in coda tre consegne; ognuna viene consegnata e ritentata in modo indipendente. Un errore su un webhook non influisce mai sugli altri.

### Cosa viene ritentato

Una consegna viene ritentata quando:

- La richiesta HTTP **non viene completata** (errore DNS, connessione rifiutata, timeout).
- Il codice di risposta HTTP è uno stato non-2xx che non è presente nell'elenco configurato dei **Codici di stato senza retry**.

Una consegna **non viene ritentata** quando:

- Il codice di risposta è `2xx` (successo).
- Il codice di risposta è presente nell'elenco configurato dei **Codici di stato senza retry**. Per impostazione predefinita questo elenco è vuoto - qualsiasi non-2xx viene ritentato.

### Configurazione dei codici senza retry

Il modulo di configurazione del webhook ha un campo **Codici di stato senza retry** (multi-valore). Voci comuni:

- `410` - Gone. Il tuo endpoint è stato spostato permanentemente o la risorsa non esiste più. Ritentare spreca solo banda da entrambe le parti.
- `422` - Unprocessable Entity. Il tuo endpoint ha compreso il payload ma lo ha considerato non valido. Ritentare con lo stesso payload darà la stessa risposta.
- `400` - Bad Request, nello stesso spirito.

Aggiungere un codice qui significa: quando l'endpoint lo restituisce, contrassegnare la consegna come fallita in modo definitivo e smettere di ritentare.

### Programma dei retry

Un worker in background viene eseguito ogni pochi secondi ed elabora le consegne il cui tempo del prossimo tentativo è trascorso.

Dopo ogni errore, il tempo del prossimo tentativo viene posticipato con **backoff lineare**: l'attesa cresce come `60 secondi * numero di tentativi` (quindi il tentativo 1 attende 1 minuto, il tentativo 2 attende 2 minuti, e così via).

Dopo 99 tentativi falliti (o 3 in sviluppo locale), la consegna viene abbandonata e rimossa dalla coda. Le voci del registro delle consegne persistono e restano visibili nella pagina [Log di consegna dei webhook](#webhook-logs) fino alla loro scadenza.

### Idempotenza da parte tua

Poiché effettuiamo retry, il tuo endpoint **deve essere idempotente**. Lo stesso `triggerId` (o `approvalId`) può arrivare più di una volta. Il tuo endpoint dovrebbe:

- Usare una chiave univoca (`triggerId` per gli eventi trigger, `approvalId` per gli eventi di approvazione) come token di deduplicazione.
- Accettare le consegne duplicate con garbo (restituire 200 la seconda volta).

Un endpoint non idempotente alla fine elaborerà due volte alcune consegne, in particolare durante interruzioni transitorie in cui un timeout viene ritentato 30 secondi dopo ma la richiesta originale in realtà ha avuto successo.

### Ordinamento

Le consegne **non sono strettamente ordinate**. Un `trigger.succeeded` e un `approval.requested` a valle (dalla stessa esecuzione) possono arrivare in qualsiasi ordine se uno viene ritentato e l'altro no. Il tuo endpoint non dovrebbe assumere un ordinamento causale.

Se hai bisogno di ordinamento, usa i timestamp - `occurredAt` sulla busta, più il `createdAt` del trigger/approvazione nel blocco dati - per ricostruire l'ordine da parte tua.

### Pulizia

Le consegne vengono rimosse dalla coda non appena hanno successo o raggiungono il limite di tentativi. La piattaforma non conserva le consegne fallite in modo definitivo nella coda stessa; il registro durevole di ogni tentativo si trova nella pagina [Log di consegna dei webhook](#webhook-logs).

### Dove guardare quando i retry falliscono

La pagina [Log di consegna dei webhook](#webhook-logs) è il posto giusto per capire perché un webhook sta fallendo. Cause comuni:

- **Errore di risoluzione DNS** - l'URL è sbagliato o il dominio non esiste più.
- **Errori TLS** - il certificato del tuo endpoint non è valido o è scaduto.
- **Connessione rifiutata / timeout** - il tuo endpoint è giù.
- **Risposte 5xx** - il tuo endpoint è attivo ma ha restituito un errore. Il corpo della risposta (troncato) viene registrato.
- **Risposte 4xx** - il tuo endpoint ha rifiutato il payload. Se è intenzionale, aggiungi il codice ai **Codici di stato senza retry**.

### Mettere in pausa un webhook non funzionante

Se un webhook fallisce costantemente, la soluzione più pulita è eliminarlo (o cancellare temporaneamente il suo elenco di iscrizioni agli eventi). La piattaforma non disabilita automaticamente i webhook che falliscono - continuano a ritentare fino a quando la consegna non viene abbandonata.