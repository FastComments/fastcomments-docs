### Pregled API-ja

Collab Chat obezbeđuje tri REST API endpointa za programsko upravljanje tekstualnim razgovorima. Ovi endpointi vam omogućavaju da preuzimate, kreirate i brišete anotacije bez upotrebe browser widgeta.

Ovo su javni endpointi koji autentifikuju korisnike putem browser kolačića. Ne koriste API ključeve. Korisnici moraju biti prijavljeni u FastComments u svom browseru da bi pristupili ovim endpointima.

### Base URL

All Collab Chat API endpoints are under:

[inline-code-attrs-start title = 'Osnovni URL'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/comment-collab-chats
[inline-code-end]

### Autentifikacija

Ovi endpointi autentifikuju korisnike putem browser kolačića. Ne koriste API ključeve. Korisnici moraju biti prijavljeni u FastComments u svom browseru da bi pristupili ovim endpointima.

### Dobavite sve razgovore

Dobavite sve tekstualne razgovore za određenu stranicu.

#### Endpoint

[inline-code-attrs-start title = 'GET Krajnja tačka'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET /comment-collab-chats/:tenantId?urlId=<urlId>
[inline-code-end]

#### Parametri

`tenantId` (parametar putanje, obavezan) je vaš FastComments Tenant ID.

`urlId` (parametar upita, obavezan) je identifikator stranice za koju želite dobaviti razgovore.

#### Odgovor

Odgovor sadrži status API-ja, informacije o trenutnoj korisničkoj sesiji ako je korisnik autentifikovan, niz razgovora sa njihovim ID-evima, URL-ovima i tekstualnim opsezima, očišćeni identifikator URL-a, indikator da li je trenutni korisnik administrator sajta ili moderator, i detalje WebSocket konekcije za sinhronizaciju uživo uključujući `tenantIdWS`, `urlIdWS`, i `userIdWS`.

#### Primer zahteva

[inline-code-attrs-start title = 'Primer GET zahteva'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl "https://fastcomments.com/comment-collab-chats/demo?urlId=my-article-page" \
  --cookie "your-session-cookie"
[inline-code-end]

#### Primer odgovora

[inline-code-attrs-start title = 'Primer GET odgovora'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Kreiranje razgovora

Kreirajte novi tekstualni razgovor za određeni izbor teksta.

#### Endpoint

[inline-code-attrs-start title = 'POST Krajnja tačka'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
POST /comment-collab-chats/:tenantId
[inline-code-end]

#### Parametri

`tenantId` (parametar putanje, obavezan) je vaš FastComments Tenant ID.

Telo zahteva mora biti u JSON formatu i sadržati sledeća obavezna polja.

`urlId` (string, obavezno) je osnovni identifikator stranice.

`urlIdWithRange` (string, obavezno) je URL kombinovan sa opsegom teksta, na primer `my-page:p:0:15,0:45{abc123}`.

`pageTitle` (string, obavezno) je naslov stranice.

`selector` (string, obavezno) je DOM putanja do elementa koji sadrži izabrani tekst.

`range` (string, obavezno) je serijalizovani tekstualni opseg u formatu `startOffset:endOffset,startOffset:endOffset{checksum}`.

`createdFromCommentId` (string, obavezno) je ID komentara koji je inicirao ovaj chat.

`broadcastId` (string, obavezno) je UUID za sinhronizaciju uživo kako bi se sprečili efekti odjeka.

#### Odgovor

Odgovor sadrži status API-ja i ID novokreiranog razgovora.

#### Primer zahteva

[inline-code-attrs-start title = 'Primer POST zahteva'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
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

#### Primer odgovora

[inline-code-attrs-start title = 'Primer POST odgovora'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "createdChatId": "conv789"
}
[inline-code-end]

### Brisanje razgovora

Obrišite postojeći tekstualni razgovor. Ovaj endpoint zahteva administratorske ili moderatorske dozvole, ili razgovor mora biti kreiran od strane autentifikovanog korisnika.

#### Endpoint

[inline-code-attrs-start title = 'DELETE Krajnja tačka'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
DELETE /comment-collab-chats/:tenantId/:chatId
[inline-code-end]

#### Parametri

`tenantId` (parametar putanje, obavezan) je vaš FastComments Tenant ID.

`chatId` (parametar putanje, obavezan) je ID razgovora koji treba obrisati.

`broadcastId` (telo zahteva, obavezno) je UUID za sinhronizaciju uživo.

#### Primer zahteva

[inline-code-attrs-start title = 'Primer DELETE zahteva'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -X DELETE "https://fastcomments.com/comment-collab-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
[inline-code-end]

#### Primer odgovora

[inline-code-attrs-start title = 'Primer DELETE odgovora'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success"
}
[inline-code-end]

### Ograničenje brzine

Ovi endpointi podložni su standardnom ograničenju stope zahteva FastComments API-ja. Middleware za ograničenje stope primenjuje se po tenant-u kako bi se sprečila zloupotreba, a istovremeno omogućili normalni obrasci korišćenja.

### Odgovori sa greškama

Svi endpointi vraćaju standardne HTTP status kodove. Odgovor 400 označava nevažeće parametre zahteva. Odgovor 401 znači da je autentifikacija neuspešna. Odgovor 403 ukazuje na nedovoljne dozvole. Odgovor 404 znači da razgovor nije pronađen. Odgovor 429 ukazuje na prekoračenje ograničenja stope.

Odgovori sa greškom uključuju JSON telo sa detaljima:

[inline-code-attrs-start title = 'Primer odgovora sa greškom'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "error",
  "message": "Conversation not found"
}
[inline-code-end]

### Praćenje upotrebe

Kreiranje razgovora povećava vašu metriku korišćenja `conversationCreateCount`. Sva WebSocket sinhronizaciona aktivnost povećava `pubSubMessageCount` i `pubSubBandwidth`. Ove metrike možete pratiti na vašem FastComments kontrolnom panelu u odeljku za analitiku upotrebe.

---