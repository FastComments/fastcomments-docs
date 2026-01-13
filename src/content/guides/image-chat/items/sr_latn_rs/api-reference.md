### Pregled API-ja

Image Chat pruža tri REST API endpointa za upravljanje razgovorima vezanim za slike programatski. Ovi endpointi vam omogućavaju da dohvatite, kreirate i brišete markere bez upotrebe widgeta u pregledaču.

Svi API endpointi zahtevaju autentifikaciju i prate standardne FastComments API obrasce. Ovo su javni endpointi koji se autentifikuju putem kolačića u pregledaču, a ne API ključeva.

### Osnovni URL

Svi Image Chat API endpointi su pod:

```
https://fastcomments.com/comment-image-chats
```

### Autentifikacija

Ovi endpointi autentifikuju korisnike putem kolačića u pregledaču. Ne koriste API ključeve. Korisnici moraju biti prijavljeni na FastComments u svom pregledaču da bi pristupili ovim endpointima.

### Dobavljanje svih razgovora

Dobavite sve razgovore vezane za određenu sliku.

#### Endpoint

```
GET /comment-image-chats/:tenantId?urlId=<urlId>
```

#### Parametri

`tenantId` (path parameter, required) je vaš FastComments Tenant ID.

`urlId` (query parameter, required) je identifikator slike za koji želite da dohvatite razgovore.

#### Odgovor

Odgovor uključuje status API-ja, informacije o trenutnoj korisničkoj sesiji ako je autentifikovan, niz razgovora sa njihovim ID-jevima, URL-ovima i X/Y koordinatama, očišćeni identifikator URL-a, zastavicu koja pokazuje da li je trenutni korisnik administrator sajta ili moderator, i detalje za WebSocket konekciju za sinhronizaciju uživo uključujući `tenantIdWS`, `urlIdWS`, i `userIdWS`.

#### Primer zahteva

```bash
curl "https://fastcomments.com/comment-image-chats/demo?urlId=my-product-image" \
  --cookie "your-session-cookie"
```

#### Primer odgovora

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

### Kreiranje razgovora

Kreirajte novi razgovor vezan za određenu lokaciju na slici.

#### Endpoint

```
POST /comment-image-chats/:tenantId
```

#### Parametri

`tenantId` (path parameter, required) je vaš FastComments Tenant ID.

Telo zahteva mora biti u JSON formatu i da sadrži sledeća obavezna polja.

`urlId` (string, required) je osnovni identifikator stranice.

`windowUrlId` (string, required) je URL kombinovan sa izvorom slike i koordinatama, na primer `my-page:/images/photo.jpg:25.5:30.2`.

`pageTitle` (string, required) je naslov stranice.

`src` (string, required) je URL izvora slike.

`x` (number, required) je X koordinata kao procenat (0-100).

`y` (number, required) je Y koordinata kao procenat (0-100).

`createdFromCommentId` (string, required) je ID komentara koji je pokrenuo ovaj razgovor.

`broadcastId` (string, required) je UUID za sinhronizaciju uživo kako bi se izbegli efekti odjeka.

#### Odgovor

Odgovor uključuje status API-ja i ID novokreiranog razgovora.

#### Primer zahteva

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

#### Primer odgovora

```json
{
  "status": "success",
  "createdChatId": "conv789"
}
```

### Brisanje razgovora

Izbrišite postojeći razgovor vezan za sliku. Ovaj endpoint zahteva administratorske ili moderatorske dozvole, ili razgovor mora biti kreiran od strane autentifikovanog korisnika.

#### Endpoint

```
DELETE /comment-image-chats/:tenantId/:chatId
```

#### Parametri

`tenantId` (path parameter, required) je vaš FastComments Tenant ID.

`chatId` (path parameter, required) je ID razgovora koji želite da obrišete.

`broadcastId` (request body, required) je UUID za sinhronizaciju uživo.

#### Primer zahteva

```bash
curl -X DELETE "https://fastcomments.com/comment-image-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
```

#### Primer odgovora

```json
{
  "status": "success"
}
```

### Sistem koordinata

X i Y koordinate se čuvaju kao procenti dimenzija slike. X predstavlja horizontalnu poziciju od leve ivice (0 = leva ivica, 100 = desna ivica). Y predstavlja vertikalnu poziciju od gornje ivice (0 = gornja ivica, 100 = donja ivica).

Ove vrednosti u procentima mogu sadržati decimale radi preciznosti. Na primer, x: 25.5 znači 25.5% od leve ivice slike.

### Ograničenje zahteva

Ovi endpointi podležu standardnom FastComments ograničenju brzine zahteva. Middleware za ograničenje stope se primenjuje po tenant-u kako bi se sprečila zloupotreba, a istovremeno omogućio normalan obrazac korišćenja.

### Odgovori sa greškom

Svi endpointi vraćaju standardne HTTP status kodove. Odgovor 400 znači neispravne parametre zahteva. Odgovor 401 znači da autentifikacija nije uspela. Odgovor 403 označava nedovoljne dozvole. Odgovor 404 znači da razgovor nije pronađen. Odgovor 429 označava prekoračenje ograničenja stope.

Odgovori sa greškom uključuju JSON telo sa detaljima:

```json
{
  "status": "error",
  "message": "Conversation not found"
}
```

### Praćenje upotrebe

Kreiranje razgovora povećava vaš usage metric `conversationCreateCount`. Sva WebSocket sinhronizacija povećava `pubSubMessageCount` i `pubSubBandwidth`. Možete pratiti ove metrike na svom FastComments kontrolnom panelu pod analitikom upotrebe.