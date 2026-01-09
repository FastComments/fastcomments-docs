### Panoramica API

Collab Chat fornisce tre endpoint REST API per gestire conversazioni testuali programmaticamente. Questi endpoint consentono di recuperare, creare ed eliminare annotazioni senza utilizzare il widget del browser.

Si tratta di endpoint pubblici che autenticano gli utenti tramite cookie del browser. Non utilizzano chiavi API. Gli utenti devono essere autenticati in FastComments nel loro browser per accedere a questi endpoint.

### URL base

Tutti gli endpoint API di Collab Chat sono sotto:

[inline-code-attrs-start title = 'URL base'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/comment-collab-chats
[inline-code-end]

### Autenticazione

Questi endpoint autenticano gli utenti tramite cookie del browser. Non utilizzano chiavi API. Gli utenti devono essere autenticati in FastComments nel loro browser per accedere a questi endpoint.

### Recupera tutte le conversazioni

Recupera tutte le conversazioni testuali per una pagina specifica.

#### Endpoint

[inline-code-attrs-start title = 'Endpoint GET'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET /comment-collab-chats/:tenantId?urlId=<urlId>
[inline-code-end]

#### Parametri

`tenantId` (parametro del percorso, obbligatorio) è il tuo FastComments Tenant ID.

`urlId` (parametro di query, obbligatorio) è l'identificatore della pagina per cui vuoi recuperare le conversazioni.

#### Risposta

La risposta include lo stato API, le informazioni sulla sessione dell'utente corrente se autenticato, un array di conversazioni con i loro ID, URL e intervalli di testo, un identificatore URL pulito, un flag che indica se l'utente corrente è amministratore del sito o moderatore, e i dettagli di connessione WebSocket per la sincronizzazione in tempo reale inclusi `tenantIdWS`, `urlIdWS`, e `userIdWS`.

#### Esempio di richiesta

[inline-code-attrs-start title = 'Esempio di richiesta GET'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl "https://fastcomments.com/comment-collab-chats/demo?urlId=my-article-page" \
  --cookie "your-session-cookie"
[inline-code-end]

#### Esempio di risposta

[inline-code-attrs-start title = 'Esempio di risposta GET'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "user": {
    "id": "user123",
    "username": "john_doe"
  },
  "conversations": [
    {
      "_id": "conv123",
      "urlId": "my-article-page:p:0:15,0:45{abc123}",
      "range": "0:15,0:45{abc123}"
    },
    {
      "_id": "conv456",
      "urlId": "my-article-page:h1:5:20,5:35{def456}",
      "range": "5:20,5:35{def456}"
    }
  ],
  "urlIdClean": "my-article-page",
  "isSiteAdmin": false,
  "tenantIdWS": "demo",
  "urlIdWS": "my-article-page",
  "userIdWS": "user123"
}
[inline-code-end]

### Crea conversazione

Crea una nuova conversazione testuale per una selezione di testo specifica.

#### Endpoint

[inline-code-attrs-start title = 'Endpoint POST'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
POST /comment-collab-chats/:tenantId
[inline-code-end]

#### Parametri

`tenantId` (parametro del percorso, obbligatorio) è il tuo FastComments Tenant ID.

Il corpo della richiesta deve essere JSON e includere questi campi obbligatori.

`urlId` (stringa, obbligatorio) è l'identificatore base della pagina.

`urlIdWithRange` (stringa, obbligatorio) è l'URL combinato con l'intervallo di testo, per esempio `my-page:p:0:15,0:45{abc123}`.

`pageTitle` (stringa, obbligatorio) è il titolo della pagina.

`selector` (stringa, obbligatorio) è il percorso DOM all'elemento che contiene il testo selezionato.

`range` (stringa, obbligatorio) è l'intervallo di testo serializzato nel formato `startOffset:endOffset,startOffset:endOffset{checksum}`.

`createdFromCommentId` (stringa, obbligatorio) è l'ID del commento che ha avviato questa chat.

`broadcastId` (stringa, obbligatorio) è un UUID per la sincronizzazione in tempo reale per prevenire effetti di eco.

#### Risposta

La risposta include lo stato API e l'ID della conversazione appena creata.

#### Esempio di richiesta

[inline-code-attrs-start title = 'Esempio di richiesta POST'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -X POST "https://fastcomments.com/comment-collab-chats/demo" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "urlId": "my-article-page",
    "urlIdWithRange": "my-article-page:p:0:15,0:45{abc123}",
    "pageTitle": "My Article Title",
    "selector": "div#article > p:nth-child(2)",
    "range": "0:15,0:45{abc123}",
    "createdFromCommentId": "comment789",
    "broadcastId": "550e8400-e29b-41d4-a716-446655440000"
  }'
[inline-code-end]

#### Esempio di risposta

[inline-code-attrs-start title = 'Esempio di risposta POST'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "createdChatId": "conv789"
}
[inline-code-end]

### Elimina conversazione

Elimina una conversazione testuale esistente. Questo endpoint richiede permessi di amministratore o moderatore, oppure la conversazione deve essere stata creata dall'utente autenticato.

#### Endpoint

[inline-code-attrs-start title = 'Endpoint DELETE'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
DELETE /comment-collab-chats/:tenantId/:chatId
[inline-code-end]

#### Parametri

`tenantId` (parametro del percorso, obbligatorio) è il tuo FastComments Tenant ID.

`chatId` (parametro del percorso, obbligatorio) è l'ID della conversazione da eliminare.

`broadcastId` (corpo della richiesta, obbligatorio) è un UUID per la sincronizzazione in tempo reale.

#### Esempio di richiesta

[inline-code-attrs-start title = 'Esempio di richiesta DELETE'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -X DELETE "https://fastcomments.com/comment-collab-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
[inline-code-end]

#### Esempio di risposta

[inline-code-attrs-start title = 'Esempio di risposta DELETE'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success"
}
[inline-code-end]

### Limitazione delle richieste

Questi endpoint sono soggetti alla limitazione delle richieste standard di FastComments. Il middleware di limitazione del rate si applica per tenant per prevenire abusi consentendo al contempo modelli di utilizzo normali.

### Risposte di errore

Tutti gli endpoint restituiscono codici di stato HTTP standard. Una risposta 400 indica parametri di richiesta non validi. Una 401 significa che l'autenticazione è fallita. Una 403 indica permessi insufficienti. Una 404 significa che la conversazione non è stata trovata. Una 429 indica che è stato superato il limite di richieste.

Le risposte di errore includono un corpo JSON con i dettagli:

[inline-code-attrs-start title = 'Esempio di risposta di errore'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "error",
  "message": "Conversation not found"
}
[inline-code-end]

### Monitoraggio dell'utilizzo

La creazione delle conversazioni incrementa la metrica di utilizzo `conversationCreateCount`. Tutta l'attività di sincronizzazione WebSocket incrementa `pubSubMessageCount` e `pubSubBandwidth`. Puoi monitorare queste metriche nella dashboard di FastComments nella sezione analytics sull'utilizzo.

---