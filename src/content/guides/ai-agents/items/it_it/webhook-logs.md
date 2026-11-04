Ogni webhook dell'agente ha il proprio log di consegna. Accessibile dalla [pagina dell'elenco dei webhook](https://fastcomments.com/auth/my-account/ai-agents/webhooks) tramite il pulsante **Logs** su ogni riga del webhook.

### Cosa c'è nella pagina

Una tabella impaginata con una riga per ogni tentativo di consegna:

| Colonna | Significato |
|---|---|
| When | Quando si è verificato il tentativo. |
| Event | Il tipo di evento (`trigger.succeeded`, `approval.requested`, ecc.). |
| Status | Lo stato della consegna. |
| StatusCode | Codice di stato HTTP restituito dal tuo endpoint, quando disponibile. |
| Description | Una breve descrizione dell'esito. Per le consegne fallite in cui non è stata ricevuta alcuna risposta HTTP, il messaggio di errore sottostante viene memorizzato come `{error: <message>}`. |

La pagina supporta solo la paginazione - non ci sono filtri per stato, tipo di evento o intervallo di date.

### Cosa puoi fare dai log

- **Decidere se un codice di stato dovrebbe essere in No-retry.** Se vedi il tuo endpoint che restituisce lo stesso `4xx` ripetutamente, questo è un segnale forte che vuoi aggiungerlo ai **No-retry status codes** così la piattaforma smette di riprovare.

### Informazioni sui fallimenti

Quando una consegna fallisce senza una risposta HTTP (errore DNS, connessione rifiutata, timeout, errore TLS, ecc.), il messaggio di errore grezzo viene registrato come `{error: <message>}`. La piattaforma non categorizza questi errori in bucket denominati come `TIMEOUT` o `DNS_ERROR` - leggi direttamente il messaggio di errore per capire cosa è successo.

Per le risposte HTTP, la colonna StatusCode mostra il codice restituito dal tuo endpoint. Casi comuni:

- **Tutti i tentativi: `401` o `403`** - il tuo endpoint sta rifiutando la firma. Verifica che stai calcolando l'HMAC su `${timestamp}.${body}` e usando il segreto corretto. Vedi [Firma dei Webhook](#webhook-signing).
- **Tutti i tentativi: `422`** - il tuo endpoint considera il payload non valido. Oppure correggi il tuo endpoint, oppure aggiungi `422` ai **No-retry status codes** se il rifiuto è previsto per alcuni eventi.

### Contesto per consegna

Ogni voce di log contiene:

- `webhookId` - quale configurazione di webhook ha prodotto questa consegna.
- `agentId` - a quale agente si riferisce la consegna.
- `triggerId` o `approvalId` - il record sottostante.
- `domain` - il dominio corrispondente.

Puoi usarli per correlare una consegna fallita con l'esecuzione a cui si riferisce nella [Cronologia delle Esecuzioni](#run-history).

### Conservazione

Le voci `AgentSyncLog` hanno un TTL fisso di 1 anno su `createdAt` indipendentemente dall'esito - le consegne riuscite e fallite vengono conservate per lo stesso periodo di tempo. Oltre la conservazione, la voce di log viene eliminata.

Se hai bisogno di un audit a lungo termine, il modello sostenibile è: far sì che **l'endpoint stesso** persisti le consegne che riceve. Questo disaccoppia il tuo log di audit dalla politica di conservazione della piattaforma.

### Invio di test

Il pulsante **Test send** del modulo di configurazione del webhook scrive una consegna fittizia nella stessa tabella di log così puoi verificare la connettività end-to-end prima di fare affidamento su eventi reali. Le consegne di test sono chiaramente etichettate nel log in modo da non inquinare le statistiche di fallimento in produzione.

### Vedi anche

- [Panoramica dei Webhook](#webhooks-overview).
- [Retry dei Webhook](#webhook-retries) per la semantica dei retry che genera questi log.
- [Firma dei Webhook](#webhook-signing) per come verificare sul tuo endpoint.