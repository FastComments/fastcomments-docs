### Pregled API-ja

Image Chat pruža tri REST API krajnje točke za programsku obradu razgovora o slikama. Ove krajnje točke omogućuju dohvat, stvaranje i brisanje markera bez korištenja widgeta u pregledniku.

Sve API krajnje točke zahtijevaju autentifikaciju i slijede standardne obrasce FastComments API-ja. Ovo su javne krajnje točke koje se autentificiraju putem kolačića preglednika, a ne putem API ključeva.

### Osnovni URL

Sve Image Chat API krajnje točke nalaze se pod:

```
https://fastcomments.com/comment-image-chats
```

### Autentifikacija

Ove krajnje točke autentificiraju korisnike putem kolačića preglednika. Ne koriste API ključeve. Korisnici moraju biti prijavljeni u FastComments u svom pregledniku kako bi imali pristup ovim krajnjim točkama.

### Dohvati sve razgovore

Dohvati sve razgovore o slici za određenu sliku.

#### Krajnja točka

```
GET /comment-image-chats/:tenantId?urlId=<urlId>
```

#### Parametri

`tenantId` (putni parametar, obavezno) je vaš FastComments Tenant ID.

`urlId` (parametar upita, obavezno) je identifikator slike za koju želite dohvatiti razgovore.

#### Odgovor

Odgovor uključuje status API-ja, informacije o trenutnoj korisničkoj sesiji ako je autentificirana, niz razgovora s njihovim ID-evima, URL-ovima i X/Y koordinatama, očišćeni identifikator URL-a, zastavicu koja označava je li trenutni korisnik administrator ili moderator stranice, te detalje za WebSocket vezu za sinkronizaciju uživo uključujući `tenantIdWS`, `urlIdWS` i `userIdWS`.

#### Primjer zahtjeva

```bash
curl "https://fastcomments.com/comment-image-chats/demo?urlId=my-product-image" \
  --cookie "your-session-cookie"
```

#### Primjer odgovora

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

### Stvaranje razgovora

Kreirajte novi razgovor o slici za određenu lokaciju na slici.

#### Krajnja točka

```
POST /comment-image-chats/:tenantId
```

#### Parametri

`tenantId` (putni parametar, obavezno) je vaš FastComments Tenant ID.

Tijelo zahtjeva mora biti JSON i mora sadržavati sljedeća obavezna polja.

`urlId` (string, obavezno) je osnovni identifikator stranice.

`windowUrlId` (string, obavezno) je URL kombiniran sa izvorom slike i koordinatama, na primjer `my-page:/images/photo.jpg:25.5:30.2`.

`pageTitle` (string, obavezno) je naslov stranice.

`src` (string, obavezno) je URL izvora slike.

`x` (number, obavezno) je X koordinata izražena kao postotak (0-100).

`y` (number, obavezno) je Y koordinata izražena kao postotak (0-100).

`createdFromCommentId` (string, obavezno) je ID komentara koji je pokrenuo ovaj razgovor.

`broadcastId` (string, obavezno) je UUID za sinkronizaciju uživo kako bi se spriječili učinci odjeka.

#### Odgovor

Odgovor uključuje status API-ja i ID novokreiranog razgovora.

#### Primjer zahtjeva

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

#### Primjer odgovora

```json
{
  "status": "success",
  "createdChatId": "conv789"
}
```

### Brisanje razgovora

Izbrišite postojeći razgovor o slici. Ova krajnja točka zahtijeva administratorske ili moderatorske dozvole, ili razgovor mora biti kreiran od strane autentificiranog korisnika.

#### Krajnja točka

```
DELETE /comment-image-chats/:tenantId/:chatId
```

#### Parametri

`tenantId` (putni parametar, obavezno) je vaš FastComments Tenant ID.

`chatId` (putni parametar, obavezno) je ID razgovora koji želite izbrisati.

`broadcastId` (tijelo zahtjeva, obavezno) je UUID za sinkronizaciju uživo.

#### Primjer zahtjeva

```bash
curl -X DELETE "https://fastcomments.com/comment-image-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
```

#### Primjer odgovora

```json
{
  "status": "success"
}
```

### Koordinatni sustav

X i Y koordinate pohranjuju se kao postotci dimenzija slike. X predstavlja horizontalnu poziciju od lijeve ruba (0 = lijevi rub, 100 = desni rub). Y predstavlja vertikalnu poziciju od gornjeg ruba (0 = gornji rub, 100 = donji rub).

Ove vrijednosti u postocima mogu uključivati decimale radi preciznosti. Na primjer, x: 25.5 znači 25.5% od lijevog ruba slike.

### Ograničenje zahtjeva

Ove krajnje točke podliježu standardnom ograničenju brzine (rate limiting) FastComments API-ja. Middleware za ograničenje primjene ograničenja radi po tenant-u kako bi se spriječila zloupotreba dok se omogućuju uobičajeni obrasci korištenja.

### Odgovori s pogreškama

Sve krajnje točke vraćaju standardne HTTP status kodove. Odgovor 400 označava nevažeće parametre zahtjeva. Odgovor 401 znači da autentifikacija nije uspjela. Odgovor 403 označava nedostatak dopuštenja. Odgovor 404 znači da razgovor nije pronađen. Odgovor 429 označava prekoračenje ograničenja brzine.

Odgovori s pogreškama uključuju JSON tijelo s detaljima:

```json
{
  "status": "error",
  "message": "Conversation not found"
}
```

### Praćenje upotrebe

Stvaranje razgovora povećava vašu metriku upotrebe `conversationCreateCount`. Sva aktivnost sinkronizacije putem WebSocket-a povećava `pubSubMessageCount` i `pubSubBandwidth`. Ove metrike možete pratiti u svom FastComments dashboardu pod analizom upotrebe.