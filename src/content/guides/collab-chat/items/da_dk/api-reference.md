### API Oversigt

Collab Chat tilbyder tre REST API-endpoints til programmatisk håndtering af tekstkonversationer. Disse endpoints giver dig mulighed for at hente, oprette og slette annotationer uden at bruge browser-widgetten.

Dette er offentlige endpoints, der autentificerer brugere via browser-cookies. De bruger ikke API-nøgler. Brugere skal være logget ind på FastComments i deres browser for at tilgå disse endpoints.

### Basis-URL

Alle Collab Chat API-endpoints findes under:

[inline-code-attrs-start title = 'Basis-URL'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/comment-collab-chats
[inline-code-end]

### Autentificering

Disse endpoints autentificerer brugere via browser-cookies. De bruger ikke API-nøgler. Brugere skal være logget ind på FastComments i deres browser for at tilgå disse endpoints.

### Hent Alle Konversationer

Hent alle tekstkonversationer for en bestemt side.

#### Endpoint

[inline-code-attrs-start title = 'GET Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET /comment-collab-chats/:tenantId?urlId=<urlId>
[inline-code-end]

#### Parametre

`tenantId` (sti-parameter, påkrævet) er dit FastComments Tenant ID.

`urlId` (query-parameter, påkrævet) er side-id'et, du vil hente konversationer for.

#### Svar

Svaret indeholder API-status, information om den aktuelle brugersession, hvis autentificeret, et array af konversationer med deres ID'er, URL'er og tekstområder, et rensset URL-identifikator, et flag, der angiver, om den aktuelle bruger er site-administrator eller moderator, samt WebSocket-forbindelsesdetaljer til live-synkronisering, herunder `tenantIdWS`, `urlIdWS` og `userIdWS`.

#### Eksempel på Forespørgsel

[inline-code-attrs-start title = 'GET Forespørgselseksempel'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl "https://fastcomments.com/comment-collab-chats/demo?urlId=my-article-page" \
  --cookie "your-session-cookie"
[inline-code-end]

#### Eksempel på Svar

[inline-code-attrs-start title = 'GET Svareksempel'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Opret Konversation

Opret en ny tekstkonversation for en bestemt tekstmarkering.

#### Endpoint

[inline-code-attrs-start title = 'POST Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
POST /comment-collab-chats/:tenantId
[inline-code-end]

#### Parametre

`tenantId` (sti-parameter, påkrævet) er dit FastComments Tenant ID.

Request-body skal være JSON og indeholde disse påkrævede felter.

`urlId` (streng, påkrævet) er basis-side-id'et.

`urlIdWithRange` (streng, påkrævet) er URL'en kombineret med tekstområdet, for eksempel `my-page:p:0:15,0:45{abc123}`.

`pageTitle` (streng, påkrævet) er sidens titel.

`selector` (streng, påkrævet) er DOM-stien til elementet, der indeholder den markerede tekst.

`range` (streng, påkrævet) er det serialiserede tekstområde i formatet `startOffset:endOffset,startOffset:endOffset{checksum}`.

`createdFromCommentId` (streng, påkrævet) er ID'et på den kommentar, der startede denne chat.

`broadcastId` (streng, påkrævet) er en UUID til live-synkronisering for at forhindre ekko-effekter.

#### Svar

Svaret indeholder API-status og ID'et på den nyoprettede konversation.

#### Eksempel på Forespørgsel

[inline-code-attrs-start title = 'POST Forespørgselseksempel'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
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

#### Eksempel på Svar

[inline-code-attrs-start title = 'POST Svareksempel'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "createdChatId": "conv789"
}
[inline-code-end]

### Slet Konversation

Slet en eksisterende tekstkonversation. Dette endpoint kræver administrator- eller moderatorrettigheder, eller at konversationen skal være oprettet af den autentificerede bruger.

#### Endpoint

[inline-code-attrs-start title = 'DELETE Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
DELETE /comment-collab-chats/:tenantId/:chatId
[inline-code-end]

#### Parametre

`tenantId` (sti-parameter, påkrævet) er dit FastComments Tenant ID.

`chatId` (sti-parameter, påkrævet) er ID'et på den konversation, der skal slettes.

`broadcastId` (request-body, påkrævet) er en UUID til live-synkronisering.

#### Eksempel på Forespørgsel

[inline-code-attrs-start title = 'DELETE Forespørgselseksempel'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -X DELETE "https://fastcomments.com/comment-collab-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
[inline-code-end]

#### Eksempel på Svar

[inline-code-attrs-start title = 'DELETE Svareksempel'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success"
}
[inline-code-end]

### Hastighedsbegrænsning

Disse endpoints er underlagt FastComments' standard API-hastighedsbegrænsning. Middlewaren for hastighedsbegrænsning anvendes pr. tenant for at forhindre misbrug, samtidig med at normale brugsmønstre tillades.

### Fejlsvær

Alle endpoints returnerer standard HTTP-statuskoder. En 400-response angiver ugyldige forespørgselsparametre. En 401-response betyder, at autentificeringen mislykkedes. En 403-response angiver utilstrækkelige rettigheder. En 404-response betyder, at konversationen ikke blev fundet. En 429-response angiver, at hastighedsgrænsen er overskredet.

Fejlsvær inkluderer en JSON-body med detaljer:

[inline-code-attrs-start title = 'Fejlsvareksempel'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "error",
  "message": "Conversation not found"
}
[inline-code-end]

### Forbrugssporing

Oprettelse af konversationer øger din `conversationCreateCount` forbrugsmetrik. Al WebSocket-synkroniseringsaktivitet øger `pubSubMessageCount` og `pubSubBandwidth`. Du kan overvåge disse metrikker i dit FastComments-dashboard under forbrugsanalyse.