Ogni webhook agente ha il proprio registro di consegna. Accessibile dalla [webhook list page](https://fastcomments.com/auth/my-account/ai-agents/webhooks) tramite il pulsante **Logs** in ogni riga del webhook.

### Cosa c'è nella pagina

Una tabella paginata con una riga per ogni tentativo di consegna:

| Colonna | Significato |
|---|---|
| Quando | Quando si è verificato il tentativo. |
| Evento | Il tipo di evento (`trigger.succeeded`, `approval.requested`, etc.). |
| Stato | Lo stato della consegna. |
| StatusCode | Codice di stato HTTP restituito dal tuo endpoint, quando disponibile. |
| Descrizione | Una breve descrizione dell'esito. Per consegne fallite dove non è stata ricevuta una risposta HTTP, il messaggio d'errore sottostante è memorizzato come `{error: <message>}`. |

La pagina supporta solo la paginazione - non ci sono filtri per stato, tipo di evento o intervallo di date.

### Cosa puoi fare dai log

- **Decidi se un codice di stato dovrebbe essere in No-retry.** Se vedi il tuo endpoint restituire lo stesso `4xx` più e più volte, è un forte segnale che vuoi aggiungerlo a **No-retry status codes** in modo che la piattaforma smetta di ritentare.

### Informazioni sui fallimenti

Quando una consegna fallisce senza una risposta HTTP (errore DNS, connessione rifiutata, timeout, errore TLS, ecc.), il messaggio d'errore grezzo viene registrato come `{error: <message>}`. La piattaforma non categorizza questi in bucket con nome come `TIMEOUT` o `DNS_ERROR` - leggi direttamente il messaggio d'errore per capire cosa è successo.

Per le risposte HTTP, la colonna StatusCode mostra il codice restituito dal tuo endpoint. Casi comuni:

- **Tutti i tentativi: `401` o `403`** - il tuo endpoint sta rifiutando la firma. Verifica che tu stia calcolando l'HMAC su `${timestamp}.${body}` e che stia usando il segreto corretto. Vedi [Firma dei webhook](#webhook-signing).
- **Tutti i tentativi: `422`** - il tuo endpoint pensa che il payload sia invalido. O correggi il tuo endpoint, oppure aggiungi `422` a **No-retry status codes** se il rifiuto è previsto per alcuni eventi.

### Contesto per ogni consegna

Ogni voce di log contiene:

- `webhookId` - quale configurazione webhook ha prodotto questa consegna.
- `agentId` - a quale agente si riferisce la consegna.
- `triggerId` or `approvalId` - il record sottostante.
- `domain` - il dominio corrispondente.

Puoi usare questi dati per correlare una consegna fallita con l'esecuzione a cui si riferisce in [Cronologia delle esecuzioni](#run-history).

### Conservazione

Le voci di `AgentSyncLog` hanno una TTL fissa di 1 anno su `createdAt` indipendentemente dall'esito - le consegne riuscite e quelle fallite vengono conservate per lo stesso periodo. Oltre il periodo di conservazione, la voce di log viene eliminata.

Se hai bisogno di audit a lungo termine, il modello sostenibile è: fare in modo che lo **stesso endpoint** persista le consegne che riceve. Questo disaccoppia il tuo registro di audit dalla politica di conservazione della piattaforma.

### Test send

Il pulsante **Test send** nel modulo di configurazione del webhook scrive una consegna finta nella stessa tabella di log così puoi verificare la connettività end-to-end prima di fare affidamento sugli eventi reali. Le consegne di test sono chiaramente etichettate nel log in modo da non inquinare le statistiche degli errori in produzione.

### Vedi anche

- [Panoramica dei Webhook](#webhooks-overview).
- [Webhook Retries](#webhook-retries) per la semantica di retry che alimenta questi log.
- [Firma dei webhook](#webhook-signing) per come verificare sul tuo endpoint.