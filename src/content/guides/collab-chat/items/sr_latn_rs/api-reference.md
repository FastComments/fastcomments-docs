### Pregled API-ja

Collab Chat pruža tri REST API endpointa za programsko upravljanje tekstualnim razgovorima. Ovi endpointi vam omogućavaju da preuzimate, kreirate i brišete anotacije bez korišćenja browser vidžeta.

Ovo su javni endpointi koji autentifikuju korisnike putem browser kolačića. Ne koriste API ključeve. Korisnici moraju biti prijavljeni na FastComments u svom browseru da bi pristupili ovim endpointima.

### Bazni URL

Svi Collab Chat API endpointi se nalaze pod:

[inline-code-attrs-start title = 'Bazni URL'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/comment-collab-chats
[inline-code-end]

### Autentifikacija

Ovi endpointi autentifikuju korisnike putem browser kolačića. Ne koriste API ključeve. Korisnici moraju biti prijavljeni na FastComments u svom browseru da bi pristupili ovim endpointima.

### Preuzimanje svih razgovora

Preuzima sve tekstualne razgovore za određenu stranicu.

#### Endpoint

[inline-code-attrs-start title = 'GET Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET /comment-collab-chats/:tenantId?urlId=<urlId>
[inline-code-end]

#### Parametri

`tenantId` (path parametar, obavezan) je vaš FastComments Tenant ID.

`urlId` (query parametar, obavezan) je identifikator stranice za koju želite da preuzmete razgovore.

#### Odgovor

Odgovor uključuje status API-ja, informacije o sesiji trenutnog korisnika ako je autentifikovan, niz razgovora sa njihovim ID-jevima, URL-ovima i tekstualnim opsezima, očišćeni URL identifikator, oznaku da li je trenutni korisnik site admin ili moderator, i detalje WebSocket konekcije za sinhronizaciju u realnom vremenu uključujući `tenantIdWS`, `urlIdWS` i `userIdWS`.

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

Kreira novi tekstualni razgovor za određenu selekciju teksta.

#### Endpoint

[inline-code-attrs-start title = 'POST Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
POST /comment-collab-chats/:tenantId
[inline-code-end]

#### Parametri

`tenantId` (path parametar, obavezan) je vaš FastComments Tenant ID.

Telo zahteva mora biti JSON i mora uključivati sledeća obavezna polja.

`urlId` (string, obavezan) je bazni identifikator stranice.

`urlIdWithRange` (string, obavezan) je URL kombinovan sa tekstualnim opsegom, na primer `my-page:p:0:15,0:45{abc123}`.

`pageTitle` (string, obavezan) je naslov stranice.

`selector` (string, obavezan) je DOM putanja do elementa koji sadrži selektovani tekst.

`range` (string, obavezan) je serijalizovani tekstualni opseg u formatu `startOffset:endOffset,startOffset:endOffset{checksum}`.

`createdFromCommentId` (string, obavezan) je ID komentara koji je inicirao ovaj razgovor.

`broadcastId` (string, obavezan) je UUID za sinhronizaciju u realnom vremenu radi sprečavanja eho efekata.

#### Odgovor

Odgovor uključuje status API-ja i ID novokreiranog razgovora.

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

Briše postojeći tekstualni razgovor. Ovaj endpoint zahteva admin ili moderator dozvole, ili razgovor mora biti kreiran od strane autentifikovanog korisnika.

#### Endpoint

[inline-code-attrs-start title = 'DELETE Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
DELETE /comment-collab-chats/:tenantId/:chatId
[inline-code-end]

#### Parametri

`tenantId` (path parametar, obavezan) je vaš FastComments Tenant ID.

`chatId` (path parametar, obavezan) je ID razgovora koji se briše.

`broadcastId` (telo zahteva, obavezan) je UUID za sinhronizaciju u realnom vremenu.

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

### Ograničenje učestalosti zahteva

Ovi endpointi podležu standardnom ograničenju učestalosti zahteva FastComments API-ja. Middleware za ograničenje učestalosti primenjuje se po tenantu radi sprečavanja zloupotrebe, uz dozvoljavanje normalnih obrazaca korišćenja.

### Odgovori o greškama

Svi endpointi vraćaju standardne HTTP statusne kodove. Odgovor 400 označava nevažeće parametre zahteva. Odgovor 401 znači da autentifikacija nije uspela. Odgovor 403 označava nedovoljne dozvole. Odgovor 404 znači da razgovor nije pronađen. Odgovor 429 označava prekoračenje ograničenja učestalosti.

Odgovori o greškama uključuju JSON telo sa detaljima:

[inline-code-attrs-start title = 'Primer odgovora o grešci'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "error",
  "message": "Conversation not found"
}
[inline-code-end]

### Praćenje korišćenja

Kreiranje razgovora povećava vašu metriku korišćenja `conversationCreateCount`. Sva WebSocket aktivnost sinhronizacije povećava `pubSubMessageCount` i `pubSubBandwidth`. Ove metrike možete pratiti na svom FastComments dashboardu u okviru analitike korišćenja.