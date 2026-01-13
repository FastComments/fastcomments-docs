### API-overzicht

Collab Chat biedt drie REST-API-eindpunten voor het programmatisch beheren van tekstgesprekken. Met deze eindpunten kunt u annotaties ophalen, maken en verwijderen zonder de browser-widget te gebruiken.

Dit zijn openbare eindpunten die gebruikers authenticeren via browsercookies. Ze gebruiken geen API-sleutels. Gebruikers moeten in hun browser zijn ingelogd bij FastComments om toegang te krijgen tot deze eindpunten.

### Basis-URL

Alle Collab Chat API-eindpunten bevinden zich onder:

[inline-code-attrs-start title = 'Basis-URL'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/comment-collab-chats
[inline-code-end]

### Authenticatie

Deze eindpunten authenticeren gebruikers via browsercookies. Ze gebruiken geen API-sleutels. Gebruikers moeten in hun browser zijn ingelogd bij FastComments om toegang te krijgen tot deze eindpunten.

### Alle gesprekken ophalen

Haal alle tekstgesprekken op voor een specifieke pagina.

#### Eindpunt

[inline-code-attrs-start title = 'GET-eindpunt'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET /comment-collab-chats/:tenantId?urlId=<urlId>
[inline-code-end]

#### Parameters

`tenantId` (padparameter, vereist) is uw FastComments Tenant-ID.

`urlId` (queryparameter, vereist) is de pagina-identificator waarvan u de gesprekken wilt ophalen.

#### Respons

De respons bevat de API-status, informatie over de huidige gebruikerssessie als er is geauthenticeerd, een array met gesprekken inclusief hun ID's, URL's en teksbereiken, een opgeschoonde URL-identificator, een vlag die aangeeft of de huidige gebruiker sitebeheerder of moderator is, en WebSocket-verbindinggegevens voor live-synchronisatie waaronder `tenantIdWS`, `urlIdWS` en `userIdWS`.

#### Voorbeeld GET-aanvraag

[inline-code-attrs-start title = 'GET-aanvraagvoorbeeld'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl "https://fastcomments.com/comment-collab-chats/demo?urlId=my-article-page" \
  --cookie "your-session-cookie"
[inline-code-end]

#### Voorbeeldrespons

[inline-code-attrs-start title = 'GET-antwoordvoorbeeld'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Gesprek aanmaken

Maak een nieuw tekstgesprek aan voor een specifieke tekstselectie.

#### Eindpunt

[inline-code-attrs-start title = 'POST-eindpunt'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
POST /comment-collab-chats/:tenantId
[inline-code-end]

#### Parameters

`tenantId` (padparameter, vereist) is uw FastComments Tenant-ID.

De requestbody moet JSON zijn en de volgende verplichte velden bevatten.

`urlId` (string, vereist) is de basis-pagina-identificator.

`urlIdWithRange` (string, vereist) is de URL gecombineerd met het tekstbereik, bijvoorbeeld `my-page:p:0:15,0:45{abc123}`.

`pageTitle` (string, vereist) is de titel van de pagina.

`selector` (string, vereist) is het DOM-pad naar het element dat de geselecteerde tekst bevat.

`range` (string, vereist) is het geserialiseerde tekstbereik in het formaat `startOffset:endOffset,startOffset:endOffset{checksum}`.

`createdFromCommentId` (string, vereist) is de ID van de opmerking die deze chat heeft gestart.

`broadcastId` (string, vereist) is een UUID voor live-synchronisatie om echo-effecten te voorkomen.

#### Respons

De respons bevat de API-status en de ID van het nieuw aangemaakte gesprek.

#### Voorbeeld POST-aanvraag

[inline-code-attrs-start title = 'POST-aanvraagvoorbeeld'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
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

#### Voorbeeldrespons

[inline-code-attrs-start title = 'POST-antwoordvoorbeeld'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "createdChatId": "conv789"
}
[inline-code-end]

### Gesprek verwijderen

Verwijder een bestaand tekstgesprek. Dit eindpunt vereist beheerder- of moderatorrechten, of het gesprek moet zijn aangemaakt door de geauthenticeerde gebruiker.

#### Eindpunt

[inline-code-attrs-start title = 'DELETE-eindpunt'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
DELETE /comment-collab-chats/:tenantId/:chatId
[inline-code-end]

#### Parameters

`tenantId` (padparameter, vereist) is uw FastComments Tenant-ID.

`chatId` (padparameter, vereist) is de ID van het gesprek dat u wilt verwijderen.

`broadcastId` (requestbody, vereist) is een UUID voor live-synchronisatie.

#### Voorbeeld DELETE-aanvraag

[inline-code-attrs-start title = 'DELETE-aanvraagvoorbeeld'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -X DELETE "https://fastcomments.com/comment-collab-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
[inline-code-end]

#### Voorbeeldrespons

[inline-code-attrs-start title = 'DELETE-antwoordvoorbeeld'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success"
}
[inline-code-end]

### Verzoeklimieten

Deze eindpunten vallen onder de standaard FastComments API-verzoeklimieten. De rate-limit middleware wordt per tenant toegepast om misbruik te voorkomen en normale gebruikspatronen toe te staan.

### Foutreacties

Alle eindpunten geven standaard HTTP-statuscodes terug. Een 400-respons duidt op ongeldige requestparameters. Een 401-respons betekent dat de authenticatie is mislukt. Een 403-respons geeft aan dat de permissies onvoldoende zijn. Een 404-respons betekent dat het gesprek niet is gevonden. Een 429-respons geeft aan dat de verzoeklimiet is overschreden.

Foutreacties bevatten een JSON-body met details:

[inline-code-attrs-start title = 'Foutantwoordvoorbeeld'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "error",
  "message": "Conversation not found"
}
[inline-code-end]

### Gebruikstracering

Het aanmaken van gesprekken verhoogt uw gebruiksmetric `conversationCreateCount`. Alle WebSocket-synchronisatie-activiteit verhoogt `pubSubMessageCount` en `pubSubBandwidth`. U kunt deze metrics volgen in uw FastComments-dashboard onder gebruiksanalyses.

---