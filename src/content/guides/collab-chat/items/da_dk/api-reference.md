### API Oversigt

Collab Chat tilbyder tre REST API-endepunkter til at administrere tekstsamtaler programmatisk. Disse endepunkter giver dig mulighed for at hente, oprette og slette annotationer uden at bruge browser-widget'en.

Disse er offentlige endepunkter, der autentificerer brugere via browser-cookies. De bruger ikke API-nøgler. Brugere skal være logget ind på FastComments i deres browser for at få adgang til disse endepunkter.

### Base URL

Alle Collab Chat API-endepunkter findes under:

[inline-code-attrs-start title = 'Base-URL'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/comment-collab-chats
[inline-code-end]

### Authentication

Disse endepunkter autentificerer brugere via browser-cookies. De bruger ikke API-nøgler. Brugere skal være logget ind på FastComments i deres browser for at få adgang til disse endepunkter.

### Get All Conversations

Hent alle tekstsamtaler for en bestemt side.

#### Endpoint

[inline-code-attrs-start title = 'GET-endepunkt'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET /comment-collab-chats/:tenantId?urlId=<urlId>
[inline-code-end]

#### Parameters

`tenantId` (sti-parameter, påkrævet) er dit FastComments Tenant ID.

`urlId` (forespørgselsparameter, påkrævet) er sideidentifikatoren, du vil hente samtaler for.

#### Response

Svaret indeholder API-status, oplysninger om den aktuelle brugers session hvis autentificeret, et array af samtaler med deres ID'er, URL'er og tekstområder, en renset URL-identifikator, et flag der angiver om den aktuelle bruger er site-admin eller moderator, samt WebSocket-forbindelsesdetaljer til live-synkronisering inklusive `tenantIdWS`, `urlIdWS`, og `userIdWS`.

#### Example Request

[inline-code-attrs-start title = 'Eksempel på GET-anmodning'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl "https://fastcomments.com/comment-collab-chats/demo?urlId=my-article-page" \
  --cookie "your-session-cookie"
[inline-code-end]

#### Example Response

[inline-code-attrs-start title = 'Eksempel på GET-svar'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Create Conversation

Opret en ny tekstsamtale for et bestemt tekstudsnit.

#### Endpoint

[inline-code-attrs-start title = 'POST-endepunkt'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
POST /comment-collab-chats/:tenantId
[inline-code-end]

#### Parameters

`tenantId` (sti-parameter, påkrævet) er dit FastComments Tenant ID.

Request-body'en skal være JSON og indeholde disse påkrævede felter.

`urlId` (string, påkrævet) er den grundlæggende sideidentifikator.

`urlIdWithRange` (string, påkrævet) er URL'en kombineret med tekstområdet, for eksempel `my-page:p:0:15,0:45{abc123}`.

`pageTitle` (string, påkrævet) er sidens titel.

`selector` (string, påkrævet) er DOM-stien til elementet, der indeholder den valgte tekst.

`range` (string, påkrævet) er det serialiserede tekstområde i formatet `startOffset:endOffset,startOffset:endOffset{checksum}`.

`createdFromCommentId` (string, påkrævet) er ID'et på kommentaren, der initierede denne chat.

`broadcastId` (string, påkrævet) er en UUID til live-synkronisering for at forhindre echo-effekter.

#### Response

Svaret inkluderer API-status og ID'et på den nyskabte samtale.

#### Example Request

[inline-code-attrs-start title = 'Eksempel på POST-anmodning'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
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

#### Example Response

[inline-code-attrs-start title = 'Eksempel på POST-svar'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "createdChatId": "conv789"
}
[inline-code-end]

### Delete Conversation

Slet en eksisterende tekstsamtale. Dette endepunkt kræver admin- eller moderatorrettigheder, eller samtalen skal være oprettet af den autentificerede bruger.

#### Endpoint

[inline-code-attrs-start title = 'DELETE-endepunkt'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
DELETE /comment-collab-chats/:tenantId/:chatId
[inline-code-end]

#### Parameters

`tenantId` (sti-parameter, påkrævet) er dit FastComments Tenant ID.

`chatId` (sti-parameter, påkrævet) er ID'et på samtalen, der skal slettes.

`broadcastId` (request body, påkrævet) er en UUID til live-synkronisering.

#### Example Request

[inline-code-attrs-start title = 'Eksempel på DELETE-anmodning'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -X DELETE "https://fastcomments.com/comment-collab-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
[inline-code-end]

#### Example Response

[inline-code-attrs-start title = 'Eksempel på DELETE-svar'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success"
}
[inline-code-end]

### Rate Limiting

Disse endepunkter er underlagt FastComments' standard ratebegrænsning for API'et. Rate limit-middleware'en gælder per-tenant for at forhindre misbrug, samtidig med at normale brugsmønstre tillades.

### Error Responses

Alle endepunkter returnerer standard HTTP-statuskoder. Et 400-svar angiver ugyldige anmodningsparametre. Et 401-svar betyder, at autentificeringen mislykkedes. Et 403-svar angiver utilstrækkelige rettigheder. Et 404-svar betyder, at samtalen ikke blev fundet. Et 429-svar angiver, at rategrænsen er overskredet.

Fejlsvar indeholder en JSON-krop med detaljer:

[inline-code-attrs-start title = 'Eksempel på fejlrespons'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "error",
  "message": "Conversation not found"
}
[inline-code-end]

### Usage Tracking

Oprettelse af samtaler øger dit `conversationCreateCount`-brugsmetrik. Al WebSocket-synkroniseringsaktivitet øger `pubSubMessageCount` og `pubSubBandwidth`. Du kan overvåge disse metrics i dit FastComments-dashboard under brugsanalyse.