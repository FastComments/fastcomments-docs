### API Oversigt

Image Chat tilbyder tre REST API-endepunkter til at administrere billedsamtaler programmatisk. Disse endepunkter gør det muligt at hente, oprette og slette markører uden at bruge browser-widget'en.

Alle API-endepunkter kræver autentificering og følger de standard FastComments API-mønstre. Dette er offentlige endepunkter, der godkender via browser-cookies, ikke API-nøgler.

### Base-URL

Alle Image Chat API-endepunkter findes under:

```
https://fastcomments.com/comment-image-chats
```

### Autentificering

Disse endepunkter godkender brugere via browser-cookies. De bruger ikke API-nøgler. Brugere skal være logget ind på FastComments i deres browser for at få adgang til disse endepunkter.

### Hent alle samtaler

Hent alle billedsamtaler for et bestemt billede.

#### Endepunkt

```
GET /comment-image-chats/:tenantId?urlId=<urlId>
```

#### Parametre

`tenantId` (sti-parameter, påkrævet) er dit FastComments Tenant ID.

`urlId` (query-parameter, påkrævet) er billedidentifikatoren, du vil hente samtaler for.

#### Respons

Responsen indeholder API-status, oplysninger om den aktuelle brugers session hvis autentificeret, et array af samtaler med deres ID'er, URL'er og X/Y-koordinater, en renset URL-identifikator, et flag der angiver om den aktuelle bruger er site-admin eller moderator, samt WebSocket-forbindelsesoplysninger til live-synkronisering inklusive `tenantIdWS`, `urlIdWS`, og `userIdWS`.

#### Eksempel på forespørgsel

```bash
curl "https://fastcomments.com/comment-image-chats/demo?urlId=my-product-image" \
  --cookie "your-session-cookie"
```

#### Eksempel på svar

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

### Opret samtale

Opret en ny billedsamtale for en bestemt placering på et billede.

#### Endepunkt

```
POST /comment-image-chats/:tenantId
```

#### Parametre

`tenantId` (sti-parameter, påkrævet) er dit FastComments Tenant ID.

Request-body'en skal være JSON og indeholde disse påkrævede felter.

`urlId` (string, påkrævet) er den grundlæggende sideidentifikator.

`windowUrlId` (string, påkrævet) er URL'en kombineret med billedkilden og koordinaterne, for eksempel `my-page:/images/photo.jpg:25.5:30.2`.

`pageTitle` (string, påkrævet) er sidens titel.

`src` (string, påkrævet) er billedets kilde-URL.

`x` (number, påkrævet) er X-koordinaten som en procent (0-100).

`y` (number, påkrævet) er Y-koordinaten som en procent (0-100).

`createdFromCommentId` (string, påkrævet) er ID'et på kommentaren, der initierede denne chat.

`broadcastId` (string, påkrævet) er en UUID til live-synkronisering for at forhindre ekko-effekter.

#### Respons

Responsen indeholder API-status og ID'et for den nyoprettede samtale.

#### Eksempel på forespørgsel

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

#### Eksempel på svar

```json
{
  "status": "success",
  "createdChatId": "conv789"
}
```

### Slet samtale

Slet en eksisterende billedsamtale. Dette endepunkt kræver admin- eller moderatorrettigheder, eller samtalen skal være oprettet af den autentificerede bruger.

#### Endepunkt

```
DELETE /comment-image-chats/:tenantId/:chatId
```

#### Parametre

`tenantId` (sti-parameter, påkrævet) er dit FastComments Tenant ID.

`chatId` (sti-parameter, påkrævet) er ID'et på samtalen, der skal slettes.

`broadcastId` (request body, påkrævet) er en UUID til live-synkronisering.

#### Eksempel på forespørgsel

```bash
curl -X DELETE "https://fastcomments.com/comment-image-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
```

#### Eksempel på svar

```json
{
  "status": "success"
}
```

### Koordinatsystem

X- og Y-koordinaterne gemmes som procenter af billedets dimensioner. X repræsenterer den vandrette position fra venstre kant (0 = venstre kant, 100 = højre kant). Y repræsenterer den lodrette position fra øverste kant (0 = øverste kant, 100 = nederste kant).

Disse procentværdier kan inkludere decimaler for præcision. For eksempel betyder x: 25.5 25.5% fra billedets venstre kant.

### Ratebegrænsning

Disse endepunkter er underlagt standard FastComments API-ratebegrænsning. Rate-limit middleware'et anvendes per-tenant for at forhindre misbrug samtidig med at normale brugsmønstre tillades.

### Fejlresponser

Alle endepunkter returnerer standard HTTP-statuskoder. Et 400-svar indikerer ugyldige forespørgselsparametre. Et 401-svar betyder, at autentificering mislykkedes. Et 403-svar indikerer utilstrækkelige rettigheder. Et 404-svar betyder, at samtalen ikke blev fundet. Et 429-svar angiver, at rategrænsen er overskredet.

Fejlresponser inkluderer en JSON-body med detaljer:

```json
{
  "status": "error",
  "message": "Conversation not found"
}
```

### Brugssporing

Oprettelse af samtaler øger din brugsmåling `conversationCreateCount`. Al WebSocket-synkroniseringsaktivitet øger `pubSubMessageCount` og `pubSubBandwidth`. Du kan overvåge disse målinger i dit FastComments-dashboard under brugsanalyse.