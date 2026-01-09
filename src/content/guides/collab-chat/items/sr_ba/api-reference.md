### Pregled API-ja

Collab Chat pruža tri REST API krajnje tačke za programatsko upravljanje tekstualnim razgovorima. Ove krajnje tačke omogućavaju dohvat, kreiranje i brisanje anotacija bez korištenja widgeta u pregledniku.

Ovo su javne krajnje tačke koje autentifikuju korisnike putem kolačića u pregledniku. Ne koriste API ključeve. Korisnici moraju biti prijavljeni u FastComments u svom pregledniku da bi pristupili ovim krajnjim tačkama.

### Osnovni URL

Sve Collab Chat API krajnje tačke se nalaze na:

[inline-code-attrs-start title = 'Osnovni URL'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/comment-collab-chats
[inline-code-end]

### Autentifikacija

Ove krajnje tačke autentifikuju korisnike putem kolačića u pregledniku. Ne koriste API ključeve. Korisnici moraju biti prijavljeni u FastComments u svom pregledniku da bi pristupili ovim krajnjim tačkama.

### Dohvati sve razgovore

Dohvati sve tekstualne razgovore za određenu stranicu.

#### Krajnja tačka

[inline-code-attrs-start title = 'GET Krajnja tačka'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET /comment-collab-chats/:tenantId?urlId=<urlId>
[inline-code-end]

#### Parametri

`tenantId` (path parameter, required) je vaš FastComments Tenant ID.

`urlId` (query parameter, required) je identifikator stranice za koju želite dohvatiti razgovore.

#### Odgovor

Odgovor uključuje status API-ja, informacije o trenutnoj korisničkoj sesiji ako je autentifikovan, niz razgovora sa njihovim ID-jevima, URL-ovima i tekstualnim rasponima, očišćen identifikator URL-a, zastavicu koja označava da li je trenutni korisnik administrator sajta ili moderator, i detalje WebSocket konekcije za sinhronizaciju uživo uključujući `tenantIdWS`, `urlIdWS`, i `userIdWS`.

#### Primjer GET zahtjeva

[inline-code-attrs-start title = 'Primjer GET zahtjeva'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl "https://fastcomments.com/comment-collab-chats/demo?urlId=my-article-page" \
  --cookie "your-session-cookie"
[inline-code-end]

#### Primjer GET odgovora

[inline-code-attrs-start title = 'Primjer GET odgovora'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Kreiraj razgovor

Kreirajte novi tekstualni razgovor za određeni izbor teksta.

#### Krajnja tačka

[inline-code-attrs-start title = 'POST Krajnja tačka'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
POST /comment-collab-chats/:tenantId
[inline-code-end]

#### Parametri

`tenantId` (path parameter, required) je vaš FastComments Tenant ID.

Tijelo zahtjeva mora biti u JSON formatu i sadržavati sljedeća obavezna polja.

`urlId` (string, required) je osnovni identifikator stranice.

`urlIdWithRange` (string, required) je URL u kombinaciji s rasponom teksta, na primjer `my-page:p:0:15,0:45{abc123}`.

`pageTitle` (string, required) je naslov stranice.

`selector` (string, required) je DOM putanja do elementa koji sadrži odabrani tekst.

`range` (string, required) je serijalizirani raspon teksta u formatu `startOffset:endOffset,startOffset:endOffset{checksum}`.

`createdFromCommentId` (string, required) je ID komentara koji je inicirao ovaj chat.

`broadcastId` (string, required) je UUID za sinhronizaciju uživo kako bi se spriječili efekti odjeka.

#### Odgovor

Odgovor uključuje status API-ja i ID novokreiranog razgovora.

#### Primjer POST zahtjeva

[inline-code-attrs-start title = 'Primjer POST zahtjeva'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
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

#### Primjer POST odgovora

[inline-code-attrs-start title = 'Primjer POST odgovora'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "createdChatId": "conv789"
}
[inline-code-end]

### Brisanje razgovora

Izbrišite postojeći tekstualni razgovor. Ova krajnja tačka zahtijeva administratorske ili moderatorske dozvole, ili razgovor mora biti kreiran od strane autentifikovanog korisnika.

#### Krajnja tačka

[inline-code-attrs-start title = 'DELETE Krajnja tačka'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
DELETE /comment-collab-chats/:tenantId/:chatId
[inline-code-end]

#### Parametri

`tenantId` (path parameter, required) je vaš FastComments Tenant ID.

`chatId` (path parameter, required) je ID razgovora koji želite izbrisati.

`broadcastId` (request body, required) je UUID za sinhronizaciju uživo.

#### Primjer DELETE zahtjeva

[inline-code-attrs-start title = 'Primjer DELETE zahtjeva'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -X DELETE "https://fastcomments.com/comment-collab-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
[inline-code-end]

#### Primjer DELETE odgovora

[inline-code-attrs-start title = 'Primjer DELETE odgovora'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success"
}
[inline-code-end]

### Ograničenja zahtjeva

Ove krajnje tačke podliježu standardnim ograničenjima brzine FastComments API-ja. Middleware za ograničenje se primjenjuje po tenant-u kako bi spriječio zloupotrebu, uz dopuštanje normalnih obrazaca korištenja.

### Odgovori o greškama

Sve krajnje tačke vraćaju standardne HTTP status kodove. Odgovor 400 označava neispravne parametre zahtjeva. Odgovor 401 znači da autentifikacija nije uspjela. Odgovor 403 označava nedostatak dozvola. Odgovor 404 znači da razgovor nije pronađen. Odgovor 429 označava prekoračenje ograničenja brzine.

Odgovori o greškama uključuju JSON tijelo s detaljima:

[inline-code-attrs-start title = 'Primjer odgovora sa greškom'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "error",
  "message": "Conversation not found"
}
[inline-code-end]

### Praćenje upotrebe

Kreiranje razgovora povećava vašu metriku upotrebe `conversationCreateCount`. Sva WebSocket sinhronizacija povećava `pubSubMessageCount` i `pubSubBandwidth`. Možete pratiti ove metrike u vašem FastComments kontrolnom panelu pod analitikom upotrebe.