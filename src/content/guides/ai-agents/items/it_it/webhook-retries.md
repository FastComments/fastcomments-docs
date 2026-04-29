Agent webhooks retry on failure. Delivery is **in modalità 'fire-and-forget' dal punto di vista dell'agente** - una consegna fallita non blocca l'esecuzione dell'agente né annulla alcuna azione - e una coda + cron gestiscono i ritentativi in modo asincrono.

### Queue model

Ogni evento viene messo in coda **una volta per webhook corrispondente**. Quindi se hai tre webhook sottoscritti a `trigger.succeeded` per un dato agente + dominio, la piattaforma mette in coda tre consegne; ciascuna viene consegnata e ritentata indipendentemente. Un fallimento su un webhook non influisce mai sugli altri.

### What's retried

Una consegna viene ritentata quando:

- La richiesta HTTP **non si completa** (errore DNS, connessione rifiutata, timeout).
- Il codice di risposta HTTP è un qualsiasi stato non 2xx che non è nella lista configurata dei **Codici di stato da non ritentare**.

Una consegna **non viene ritentata** quando:

- Il codice di risposta è `2xx` (successo).
- Il codice di risposta è nella lista configurata dei **Codici di stato da non ritentare**. Per impostazione predefinita questa lista è vuota - qualsiasi non-2xx viene ritentato.

### Configuring no-retry codes

Il modulo di configurazione del webhook ha un campo **Codici di stato da non ritentare** (valori multipli). Voci comuni:

- `410` - Gone. Il tuo endpoint è stato spostato in modo permanente o la risorsa non esiste più. Ritentare sarebbe solo uno spreco di larghezza di banda per entrambe le parti.
- `422` - Unprocessable Entity. Il tuo endpoint ha compreso il payload ma lo ha considerato non valido. Ritentare con lo stesso payload restituirà la stessa risposta.
- `400` - Bad Request, nello stesso spirito.

Aggiungere un codice qui significa: quando l'endpoint lo restituisce, contrassegna la consegna come failed-terminal e interrompi i ritentativi.

### Retry schedule

Un worker in background gira ogni pochi secondi e processa tutte le consegne il cui prossimo tentativo è passato.

Dopo ogni fallimento, il tempo del prossimo tentativo viene spostato in avanti con **backoff lineare**: l'attesa cresce come `60 seconds * attempt count` (quindi il tentativo 1 attende 1 minuto, il tentativo 2 attende 2 minuti, e così via).

Dopo 99 tentativi falliti (o 3 in sviluppo locale), la consegna viene abbandonata e rimossa dalla coda. Le voci del registro delle consegne persistono e rimangono visibili nella pagina [Registri delle consegne webhook](#webhook-logs) fino alla loro scadenza.

### Idempotence on your side

Poiché effettuiamo ritentativi, il tuo endpoint **deve essere idempotente**. Lo stesso `triggerId` (o `approvalId`) può arrivare più di una volta. Il tuo endpoint dovrebbe:

- Usare una chiave unica (`triggerId` per gli eventi trigger, `approvalId` per gli eventi di approval) come token di deduplicazione.
- Accettare consegne duplicate in modo elegante (restituire 200 la seconda volta).

Un endpoint non idempotente alla fine elaborerà alcune consegne due volte, specialmente durante interruzioni transitorie in cui un timeout viene ritentato 30 secondi dopo ma la richiesta originale è effettivamente riuscita.

### Ordering

Le consegne **non sono strettamente ordinate**. Un `trigger.succeeded` e un `approval.requested` a valle (dello stesso run) possono arrivare in ordine diverso se uno viene ritentato e l'altro no. Il tuo endpoint non dovrebbe presumere un ordinamento causale.

Se hai bisogno di ordine, usa i timestamp - `occurredAt` sull'envelope, più il `createdAt` del trigger/approval nel blocco data - per ricostruire l'ordine dalla tua parte.

### Cleanup

Le consegne vengono rimosse dalla coda non appena o riescono o raggiungono il limite di tentativi. La piattaforma non conserva le consegne terminalmente fallite nella coda stessa; il record duraturo di ogni tentativo vive nella pagina [Registri delle consegne webhook](#webhook-logs).

### Where to look when retries fail

La pagina [Registri delle consegne webhook](#webhook-logs) è il posto dove vedere perché un webhook sta fallendo. Cause comuni:

- **Errore di risoluzione DNS** - l'URL è sbagliato o il dominio non esiste più.
- **Errori TLS** - il certificato del tuo endpoint è invalido o scaduto.
- **Connessione rifiutata / timeout** - il tuo endpoint è down.
- **Risposte 5xx** - il tuo endpoint è raggiungibile ma ha generato un errore. Il corpo della risposta (troncato) viene registrato.
- **Risposte 4xx** - il tuo endpoint ha rifiutato il payload. Se questo è intenzionale, aggiungi il codice ai **Codici di stato da non ritentare**.

### Pause an unhealthy webhook

Se un webhook fallisce costantemente, la soluzione più pulita è eliminarlo (o svuotare temporaneamente la sua lista di sottoscrizione agli eventi). La piattaforma non disabilita automaticamente i webhook che falliscono - continueranno a ritentare finché la consegna non viene abbandonata.