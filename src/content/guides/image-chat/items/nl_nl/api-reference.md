### API-overzicht

Image Chat biedt drie REST API-endpoints voor het programmatisch beheren van afbeeldingsgesprekken. Met deze endpoints kunt u markers ophalen, aanmaken en verwijderen zonder de browser-widget te gebruiken.

Alle API-endpoints vereisen authenticatie en volgen de standaard FastComments API-patronen. Dit zijn publieke endpoints die via browsercookies worden geauthenticeerd, niet via API-sleutels.

### Basis-URL

Alle Image Chat API-endpoints vallen onder:

```
https://fastcomments.com/comment-image-chats
```

### Authenticatie

Deze endpoints authenticeren gebruikers via browsercookies. Ze gebruiken geen API-sleutels. Gebruikers moeten in hun browser ingelogd zijn bij FastComments om toegang te krijgen tot deze endpoints.

### Alle gesprekken ophalen

Haal alle afbeeldingsgesprekken op voor een specifieke afbeelding.

#### Endpoint

```
GET /comment-image-chats/:tenantId?urlId=<urlId>
```

#### Parameters

`tenantId` (padparameter, vereist) is uw FastComments Tenant ID.

`urlId` (queryparameter, vereist) is de afbeeldingsidentificatie waarvoor u gesprekken wilt ophalen.

#### Respons

De respons bevat de API-status, informatie over de huidige gebruikerssessie indien geauthenticeerd, een array met gesprekken inclusief hun ID's, URL's en X/Y-coördinaten, een opgeschoonde URL-identificatie, een vlag die aangeeft of de huidige gebruiker een sitebeheerder of moderator is, en WebSocket-verbindingsdetails voor live synchronisatie, waaronder `tenantIdWS`, `urlIdWS` en `userIdWS`.

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

Maak een nieuw afbeeldingsgesprek aan voor een specifieke locatie op een afbeelding.

#### Endpoint

```
POST /comment-image-chats/:tenantId
```

#### Parameters

`tenantId` (padparameter, vereist) is uw FastComments Tenant ID.

De request body moet JSON zijn en deze verplichte velden bevatten.

`urlId` (string, verplicht) is de basispagina-identificatie.

`windowUrlId` (string, verplicht) is de URL gecombineerd met de afbeeldingsbron en coördinaten, bijvoorbeeld `my-page:/images/photo.jpg:25.5:30.2`.

`pageTitle` (string, verplicht) is de titel van de pagina.

`src` (string, verplicht) is de URL van de afbeeldingsbron.

`x` (getal, verplicht) is de X-coördinaat als percentage (0-100).

`y` (getal, verplicht) is de Y-coördinaat als percentage (0-100).

`createdFromCommentId` (string, verplicht) is de ID van de reactie die deze chat heeft gestart.

`broadcastId` (string, verplicht) is een UUID voor live synchronisatie om echo-effecten te voorkomen.

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

Verwijder een bestaand afbeeldingsgesprek. Dit endpoint vereist beheerder- of moderatorrechten, of het gesprek moet zijn aangemaakt door de geauthenticeerde gebruiker.

#### Endpoint

```
DELETE /comment-image-chats/:tenantId/:chatId
```

#### Parameters

`tenantId` (padparameter, vereist) is uw FastComments Tenant ID.

`chatId` (padparameter, vereist) is de ID van het te verwijderen gesprek.

`broadcastId` (request body, vereist) is een UUID voor live synchronisatie.

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

### Coördinatensysteem

De X- en Y-coördinaten worden opgeslagen als percentages van de afmetingen van de afbeelding. X vertegenwoordigt de horizontale positie vanaf de linkerrand (0 = linkerrand, 100 = rechterrand). Y vertegenwoordigt de verticale positie vanaf de bovenrand (0 = bovenrand, 100 = onderrand).

Deze percentagewaarden kunnen decimalen bevatten voor precisie. Bijvoorbeeld, x: 25.5 betekent 25,5% vanaf de linkerrand van de afbeelding.

### Rate limiting

Deze endpoints vallen onder de standaard FastComments API rate limiting. De rate limit-middleware geldt per tenant om misbruik te voorkomen terwijl normaal gebruik mogelijk blijft.

### Foutresponsen

Alle endpoints geven standaard HTTP-statuscodes terug. Een 400-respons geeft ongeldige verzoekparameters aan. Een 401-respons betekent dat de authenticatie is mislukt. Een 403-respons geeft onvoldoende rechten aan. Een 404-respons betekent dat het gesprek niet is gevonden. Een 429-respons geeft aan dat de rate limit is overschreden.

Foutresponsen bevatten een JSON-body met details:

```json
{
  "status": "error",
  "message": "Conversation not found"
}
```

### Gebruiksbewaking

Het aanmaken van gesprekken verhoogt uw `conversationCreateCount` gebruiksstatistiek. Alle WebSocket-synchronisatieactiviteit verhoogt `pubSubMessageCount` en `pubSubBandwidth`. U kunt deze statistieken monitoren in uw FastComments-dashboard onder gebruiksanalyse.