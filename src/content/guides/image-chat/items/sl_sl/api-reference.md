### API Overview

Image Chat zagotavlja tri REST API končne točke za upravljanje pogovorov o slikah programatično. Te končne točke vam omogočajo pridobivanje, ustvarjanje in brisanje oznak brez uporabe vtičnika v brskalniku.

Vse API končne točke zahtevajo overjanje in sledijo standardnim vzorcem FastComments API. To so javne končne točke, ki se overjajo preko piškotkov brskalnika, ne preko API ključev.

### Base URL

Vse Image Chat API končne točke so pod:

```
https://fastcomments.com/comment-image-chats
```

### Authentication

Te končne točke overjajo uporabnike preko piškotkov v brskalniku. Ne uporabljajo API ključev. Uporabniki morajo biti v svojem brskalniku prijavljeni v FastComments, da lahko dostopajo do teh končnih točk.

### Get All Conversations

Pridobite vse pogovore o sliki za določeno sliko.

#### Endpoint

```
GET /comment-image-chats/:tenantId?urlId=<urlId>
```

#### Parameters

`tenantId` (path parameter, required) je vaš FastComments Tenant ID.

`urlId` (query parameter, required) je identifikator slike, za katero želite pridobiti pogovore.

#### Response

Odgovor vključuje stanje API, informacije o trenutni uporabniški seji, če je overjena, polje z nizi pogovorov z njihovimi ID-ji, URL-ji in X/Y koordinatami, očiščen identifikator URL, zastavico, ki označuje ali je trenutni uporabnik skrbnik spletnega mesta ali moderator, in podrobnosti povezave WebSocket za sinhronizacijo v živo vključno z `tenantIdWS`, `urlIdWS` in `userIdWS`.

#### Example Request

```bash
curl "https://fastcomments.com/comment-image-chats/demo?urlId=my-product-image" \
  --cookie "your-session-cookie"
```

#### Example Response

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

### Create Conversation

Ustvarite nov pogovor o sliki za določeno lokacijo na sliki.

#### Endpoint

```
POST /comment-image-chats/:tenantId
```

#### Parameters

`tenantId` (path parameter, required) je vaš FastComments Tenant ID.

Telo zahteve mora biti v formatu JSON in mora vsebovati ta obvezna polja.

`urlId` (string, required) je osnovni identifikator strani.

`windowUrlId` (string, required) je URL združen z izvorom slike in koordinatami, na primer `my-page:/images/photo.jpg:25.5:30.2`.

`pageTitle` (string, required) je naslov strani.

`src` (string, required) je URL vira slike.

`x` (number, required) je X koordinata kot odstotek (0-100).

`y` (number, required) je Y koordinata kot odstotek (0-100).

`createdFromCommentId` (string, required) je ID komentarja, ki je sprožil ta klepet.

`broadcastId` (string, required) je UUID za sinhronizacijo v živo, da prepreči efekt odmeva.

#### Response

Odgovor vključuje stanje API in ID novo ustvarjenega pogovora.

#### Example Request

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

#### Example Response

```json
{
  "status": "success",
  "createdChatId": "conv789"
}
```

### Delete Conversation

Izbrišite obstoječ pogovor o sliki. Ta končna točka zahteva skrbniške ali moderatorske pravice ali pa mora biti pogovor ustvaril overjeni uporabnik.

#### Endpoint

```
DELETE /comment-image-chats/:tenantId/:chatId
```

#### Parameters

`tenantId` (path parameter, required) je vaš FastComments Tenant ID.

`chatId` (path parameter, required) je ID pogovora, ki ga želite izbrisati.

`broadcastId` (request body, required) je UUID za sinhronizacijo v živo.

#### Example Request

```bash
curl -X DELETE "https://fastcomments.com/comment-image-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
```

#### Example Response

```json
{
  "status": "success"
}
```

### Coordinate System

Koordinate X in Y so shranjene kot odstotki dimenzij slike. X predstavlja horizontalni položaj od levega roba (0 = levi rob, 100 = desni rob). Y predstavlja vertikalni položaj od zgornjega roba (0 = zgornji rob, 100 = spodnji rob).

Ti odstotni vrednosti lahko vključujejo decimalke za natančnost. Na primer, x: 25.5 pomeni 25.5% od levega roba slike.

### Rate Limiting

Te končne točke so predmet standardnega omejevanja zahtev FastComments API. Middleware za omejevanje hitrosti se uporablja na nivoju najemnika (tenant), da prepreči zlorabe, hkrati pa omogoči normalne vzorce uporabe.

### Error Responses

Vse končne točke vračajo standardne HTTP statusne kode. Odgovor 400 pomeni neveljavne parametre zahteve. Odgovor 401 pomeni, da overjanje ni uspelo. Odgovor 403 pomeni nezadostne pravice. Odgovor 404 pomeni, da pogovor ni bil najden. Odgovor 429 pomeni preseženo omejitev zahtev.

Odgovori z napako vključujejo JSON telo s podrobnostmi:

```json
{
  "status": "error",
  "message": "Conversation not found"
}
```

### Usage Tracking

Ustvarjanje pogovorov poveča vašo metriko uporabe `conversationCreateCount`. Vsa WebSocket sinhronizacija poveča `pubSubMessageCount` in `pubSubBandwidth`. Te metrike lahko spremljate v nadzorni plošči FastComments pod analitiko uporabe.