### API Overzicht

Image Chat biedt drie REST API-eindpunten voor het programatisch beheren van gesprekken bij afbeeldingen. Deze eindpunten stellen je in staat gesprekken op te halen, aan te maken en te verwijderen zonder de browserwidget te gebruiken.

Alle API-eindpunten vereisen authenticatie en volgen de standaard FastComments API-patronen. Dit zijn openbare eindpunten die authenticeren via browsercookies, niet via API-sleutels.

### Basis-URL

Alle Image Chat API-eindpunten bevinden zich onder:

```
https://fastcomments.com/comment-image-chats
```

### Authenticatie

Deze eindpunten authenticeren gebruikers via browsercookies. Ze gebruiken geen API-sleutels. Gebruikers moeten ingelogd zijn bij FastComments in hun browser om toegang te krijgen tot deze eindpunten.

### Alle gesprekken ophalen

Haal alle beeldgesprekken op voor een specifieke afbeelding.

#### Eindpunt

```
GET /comment-image-chats/:tenantId?urlId=<urlId>
```

#### Parameters

`tenantId` (padparameter, verplicht) is je FastComments Tenant ID.

`urlId` (queryparameter, verplicht) is de afbeeldingsidentificatie waarvoor je gesprekken wilt ophalen.

#### Respons

De respons bevat de API-status, informatie over de huidige gebruikerssessie als geauthenticeerd, een array met gesprekken inclusief hun IDs, URL's en X/Y-coördinaten, een opgeschoonde URL-identificatie, een vlag die aangeeft of de huidige gebruiker sitebeheerder of moderator is, en WebSocket-verbindinggegevens voor live-synchronisatie inclusief `tenantIdWS`, `urlIdWS`, en `userIdWS`.

#### Voorbeeldverzoek

```bash
curl "https://fastcomments.com/comment-image-chats/demo?urlId=my-product-image" \
  --cookie "your-session-cookie"
```

#### Voorbeeldrespons

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

### Gesprek aanmaken

Maak een nieuw beeldgesprek aan voor een specifieke locatie op een afbeelding.

#### Eindpunt

```
POST /comment-image-chats/:tenantId
```

#### Parameters

`tenantId` (padparameter, verplicht) is je FastComments Tenant ID.

De request body moet JSON zijn en de volgende verplichte velden bevatten.

`urlId` (string, verplicht) is de basispagina-identificatie.

`windowUrlId` (string, verplicht) is de URL gecombineerd met de afbeeldingsbron en coördinaten, bijvoorbeeld `my-page:/images/photo.jpg:25.5:30.2`.

`pageTitle` (string, verplicht) is de titel van de pagina.

`src` (string, verplicht) is de afbeeldingsbron-URL.

`x` (number, verplicht) is de X-coördinaat als percentage (0-100).

`y` (number, verplicht) is de Y-coördinaat als percentage (0-100).

`createdFromCommentId` (string, verplicht) is de ID van de opmerking die dit gesprek heeft gestart.

`broadcastId` (string, verplicht) is een UUID voor live-synchronisatie om echo-effecten te voorkomen.

#### Respons

De respons bevat de API-status en de ID van het nieuw aangemaakte gesprek.

#### Voorbeeldverzoek

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

#### Voorbeeldrespons

```json
{
  "status": "success",
  "createdChatId": "conv789"
}
```

### Gesprek verwijderen

Verwijder een bestaand beeldgesprek. Dit eindpunt vereist admin- of moderatorrechten, of het gesprek moet zijn aangemaakt door de geauthenticeerde gebruiker.

#### Eindpunt

```
DELETE /comment-image-chats/:tenantId/:chatId
```

#### Parameters

`tenantId` (padparameter, verplicht) is je FastComments Tenant ID.

`chatId` (padparameter, verplicht) is de ID van het gesprek dat verwijderd moet worden.

`broadcastId` (request body, verplicht) is een UUID voor live-synchronisatie.

#### Voorbeeldverzoek

```bash
curl -X DELETE "https://fastcomments.com/comment-image-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
```

#### Voorbeeldrespons

```json
{
  "status": "success"
}
```

### Coördinatenstelsel

De X- en Y-coördinaten worden opgeslagen als percentages van de afbeeldingsafmetingen. X geeft de horizontale positie vanaf de linkerkant aan (0 = linker rand, 100 = rechter rand). Y geeft de verticale positie vanaf de bovenkant aan (0 = bovenrand, 100 = onderrand).

Deze procentwaarden kunnen decimalen bevatten voor precisie. Bijvoorbeeld, x: 25.5 betekent 25.5% vanaf de linkerrand van de afbeelding.

### Rate Limiting

Deze eindpunten vallen onder de standaard FastComments API rate limiting. De rate limit-middleware wordt per tenant toegepast om misbruik te voorkomen en tegelijk normaal gebruik toe te staan.

### Foutantwoorden

Alle eindpunten geven standaard HTTP-statuscodes terug. Een 400-respons duidt op ongeldige aanvraagparameters. Een 401-respons betekent dat authenticatie is mislukt. Een 403-respons geeft aan dat de machtigingen onvoldoende zijn. Een 404-respons betekent dat het gesprek niet is gevonden. Een 429-respons geeft aan dat het tarieflimiet is overschreden.

Foutantwoorden bevatten een JSON-body met details:

```json
{
  "status": "error",
  "message": "Conversation not found"
}
```

### Gebruikstracering

Het aanmaken van gesprekken verhoogt je `conversationCreateCount` gebruiksmetric. Alle WebSocket-synchronisatie-activiteit verhoogt `pubSubMessageCount` en `pubSubBandwidth`. Je kunt deze metrics monitoren in je FastComments-dashboard onder gebruiksanalyse.