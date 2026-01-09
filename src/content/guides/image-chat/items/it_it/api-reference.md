### Panoramica API

Image Chat fornisce tre endpoint REST API per gestire le conversazioni sulle immagini in modo programmatico. Questi endpoint consentono di recuperare, creare ed eliminare i marcatori senza utilizzare il widget del browser.

Tutti gli endpoint API richiedono l'autenticazione e seguono i modelli standard dell'API FastComments. Si tratta di endpoint pubblici che autenticano tramite cookie del browser, non con API key.

### URL base

Tutti gli endpoint Image Chat API si trovano sotto:

```
https://fastcomments.com/comment-image-chats
```

### Autenticazione

Questi endpoint autenticano gli utenti tramite cookie del browser. Non usano API key. Gli utenti devono essere autenticati su FastComments nel loro browser per accedere a questi endpoint.

### Recupera tutte le conversazioni

Recupera tutte le conversazioni per una specifica immagine.

#### Endpoint

```
GET /comment-image-chats/:tenantId?urlId=<urlId>
```

#### Parametri

`tenantId` (parametro di percorso, obbligatorio) è il tuo Tenant ID di FastComments.

`urlId` (parametro di query, obbligatorio) è l'identificatore dell'immagine per cui vuoi recuperare le conversazioni.

#### Risposta

La risposta include lo stato dell'API, le informazioni sulla sessione dell'utente corrente se autenticato, un array di conversazioni con i loro ID, URL e coordinate X/Y, un identificatore URL pulito, un flag che indica se l'utente corrente è amministratore del sito o moderatore, e i dettagli di connessione WebSocket per la sincronizzazione in tempo reale inclusi `tenantIdWS`, `urlIdWS` e `userIdWS`.

#### Esempio di richiesta

```bash
curl "https://fastcomments.com/comment-image-chats/demo?urlId=my-product-image" \
  --cookie "your-session-cookie"
```

#### Esempio di risposta

```json
{
  "status": "success",
  "user": {
    "id": "user123",
    "username": "john_doe"
  },
  "conversations": [
    {
      "_id": "conv123",
      "urlId": "my-product-image:/images/product.jpg:25:30",
      "x": 25.5,
      "y": 30.2
    },
    {
      "_id": "conv456",
      "urlId": "my-product-image:/images/product.jpg:60:45",
      "x": 60.8,
      "y": 45.1
    }
  ],
  "urlIdClean": "my-product-image",
  "isSiteAdmin": false,
  "tenantIdWS": "demo",
  "urlIdWS": "my-product-image",
  "userIdWS": "user123"
}
```

### Crea conversazione

Crea una nuova conversazione sull'immagine in una posizione specifica.

#### Endpoint

```
POST /comment-image-chats/:tenantId
```

#### Parametri

`tenantId` (parametro di percorso, obbligatorio) è il tuo Tenant ID di FastComments.

Il corpo della richiesta deve essere in JSON e includere i seguenti campi obbligatori.

`urlId` (stringa, obbligatorio) è l'identificatore base della pagina.

`windowUrlId` (stringa, obbligatorio) è l'URL combinato con la sorgente dell'immagine e le coordinate, per esempio `my-page:/images/photo.jpg:25.5:30.2`.

`pageTitle` (stringa, obbligatorio) è il titolo della pagina.

`src` (stringa, obbligatorio) è l'URL della sorgente dell'immagine.

`x` (numero, obbligatorio) è la coordinata X come percentuale (0-100).

`y` (numero, obbligatorio) è la coordinata Y come percentuale (0-100).

`createdFromCommentId` (stringa, obbligatorio) è l'ID del commento che ha avviato questa chat.

`broadcastId` (stringa, obbligatorio) è un UUID per la sincronizzazione in tempo reale per prevenire effetti di eco.

#### Risposta

La risposta include lo stato dell'API e l'ID della conversazione appena creata.

#### Esempio di richiesta

```bash
curl -X POST "https://fastcomments.com/comment-image-chats/demo" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "urlId": "my-product-image",
    "windowUrlId": "my-product-image:/images/product.jpg:25.5:30.2",
    "pageTitle": "Product Gallery",
    "src": "/images/product.jpg",
    "x": 25.5,
    "y": 30.2,
    "createdFromCommentId": "comment789",
    "broadcastId": "550e8400-e29b-41d4-a716-446655440000"
  }'
```

#### Esempio di risposta

```json
{
  "status": "success",
  "createdChatId": "conv789"
}
```

### Elimina conversazione

Elimina una conversazione esistente sull'immagine. Questo endpoint richiede permessi di amministratore o moderatore, oppure la conversazione deve essere stata creata dall'utente autenticato.

#### Endpoint

```
DELETE /comment-image-chats/:tenantId/:chatId
```

#### Parametri

`tenantId` (parametro di percorso, obbligatorio) è il tuo Tenant ID di FastComments.

`chatId` (parametro di percorso, obbligatorio) è l'ID della conversazione da eliminare.

`broadcastId` (corpo della richiesta, obbligatorio) è un UUID per la sincronizzazione in tempo reale.

#### Esempio di richiesta

```bash
curl -X DELETE "https://fastcomments.com/comment-image-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
```

#### Esempio di risposta

```json
{
  "status": "success"
}
```

### Sistema di coordinate

Le coordinate X e Y sono memorizzate come percentuali rispetto alle dimensioni dell'immagine. X rappresenta la posizione orizzontale a partire dal bordo sinistro (0 = bordo sinistro, 100 = bordo destro). Y rappresenta la posizione verticale a partire dal bordo superiore (0 = bordo superiore, 100 = bordo inferiore).

Questi valori percentuali possono includere decimali per una maggiore precisione. Ad esempio, x: 25.5 significa 25.5% dal bordo sinistro dell'immagine.

### Limitazione delle richieste

Questi endpoint sono soggetti alla limitazione della frequenza standard dell'API FastComments. Il middleware per il rate limit si applica per tenant per prevenire abusi consentendo al contempo schemi di utilizzo normali.

### Risposte di errore

Tutti gli endpoint restituiscono i codici di stato HTTP standard. Una risposta 400 indica parametri di richiesta non validi. Una risposta 401 significa che l'autenticazione è fallita. Una risposta 403 indica permessi insufficienti. Una risposta 404 significa che la conversazione non è stata trovata. Una risposta 429 indica che è stato superato il limite di richieste.

Le risposte di errore includono un body JSON con i dettagli:

```json
{
  "status": "error",
  "message": "Conversation not found"
}
```

### Monitoraggio dell'utilizzo

La creazione di conversazioni incrementa la tua metrica di utilizzo `conversationCreateCount`. Tutta l'attività di sincronizzazione WebSocket incrementa `pubSubMessageCount` e `pubSubBandwidth`. Puoi monitorare queste metriche nella tua dashboard FastComments nella sezione analytics di utilizzo.