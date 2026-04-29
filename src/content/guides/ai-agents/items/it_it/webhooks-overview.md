Agent webhooks sono callback HTTP attivati dalla piattaforma quando un'esecuzione di un agente si completa o quando lo stato di un'approvazione cambia. Usali per inoltrare l'attività degli agenti nei tuoi sistemi — dashboard di moderazione, log di audit, canali Slack, strumenti di escalation.

Configurato sotto la scheda **Webhooks** nella [pagina Agenti AI](https://fastcomments.com/auth/my-account/ai-agents).

### Cosa viene inviato

Quattro tipi di evento:

- **`trigger.succeeded`** - un'esecuzione dell'agente si è completata con successo.
- **`trigger.failed`** - un'esecuzione dell'agente ha generato un errore.
- **`approval.requested`** - un'azione è stata messa in coda per l'approvazione umana.
- **`approval.decided`** - un'approvazione è stata approvata, rifiutata o l'esecuzione è fallita.

Vedi [Eventi Webhook](#webhook-events) per sapere quali eventi vengono generati quando, e [Payload dei Webhook](#webhook-payloads) per lo schema di ciascuno.

### Cosa non viene inviato

- **Per-tool-action webhooks.** Un'esecuzione che invoca `pin_comment` non genera un webhook separato per il pin — l'azione è inclusa nel payload `trigger.succeeded` dell'esecuzione. Se vuoi la consegna per singola azione, analizza l'array `actions` nel payload del trigger.
- **Trigger scartati.** Un trigger che non viene dispatchato (budget esaurito, scope errato) non genera un webhook. I drop sono visibili solo nella [pagina Analytics](#analytics-page).
- **Trigger prodotti da replay.** Le esecuzioni di test non generano webhook. Vedi [Esecuzioni di test (Replay)](#test-runs-replays).

### Configurazione

Per ogni webhook che imposti:

- **URL** - l'endpoint HTTPS su cui fare POST.
- **Domain** - il dominio dei commenti a cui si applica questo webhook (il tuo tenant può ospitare più domini). `*` corrisponde a tutti i domini; `*.example.com` è un glob; un dominio esatto corrisponde solo a quello.
- **Events** - a quali dei quattro tipi di evento iscriversi.
- **Agents** - vuoto per "tutti gli agenti", oppure una lista di ID agente specifici per filtrare.
- **Method** - POST o PUT (POST di default).
- **No-retry status codes** - codici di risposta HTTP che devono essere trattati come errori terminali, non ritentati (ad es., 410 Gone, 422 Unprocessable). Vedi [Retry dei Webhook](#webhook-retries).

Più webhook possono iscriversi allo stesso evento — ciascuno viene messo in coda indipendentemente e consegnato al proprio URL.

### Corrispondenza per dominio

Un dato evento viene consegnato a **ogni webhook il cui campo `domain` corrisponde al dominio del commento**. La corrispondenza utilizza un semplice glob:

- Esatto: `example.com` corrisponde solo a `example.com`.
- Asterisco wildcard: `*` corrisponde a ogni dominio.
- Glob per sottodominio: `*.example.com` corrisponde a `blog.example.com`, `forum.example.com`, ma non a `example.com` stesso.

Il dominio in un payload è il primo risultato non nullo tra: il `domain` del commento, l'URL analizzata rispetto alla configurazione dei domini del tuo tenant, o la ricerca `Page` tramite `urlId`.

### Filtraggio per agente

Il campo **Agents** permette a un webhook di iscriversi solo a determinati agenti. Vuoto significa "tutti gli agenti". Quando non è vuoto, il webhook si attiva solo per gli agenti nella lista.

Questo è utile quando hai un webhook per eventi di moderazione e un altro per eventi di engagement, entrambi instradati verso sistemi downstream differenti.

### Invio di test

L'interfaccia di configurazione del webhook ha un pulsante **Test send** che invia un payload fittizio all'URL così puoi verificare connettività, firma e il codice di risposta del tuo endpoint senza aspettare un evento reale.

### Log di consegna

Ogni consegna (e ogni ritentativo) finisce nella pagina dei [Log di consegna dei Webhook](#webhook-logs) così puoi vedere cosa è successo sulla rete.

### Firma

Ogni webhook viene firmato con HMAC-SHA256 usando il secret API del tuo tenant. Vedi [Firma dei Webhook](#webhook-signing).

### Idoneità

I webhook degli agenti richiedono fatturazione valida sul tenant. Usano la stessa infrastruttura di firma/secret dei tuoi webhook dei commenti esistenti — se hai già integrato i webhook dei commenti (vedi la [Guida ai Webhook](/guide-webhooks.html)), l'integrazione dei webhook degli agenti ha la stessa struttura con nuovi tipi di evento.