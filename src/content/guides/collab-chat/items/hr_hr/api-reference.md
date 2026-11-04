### Pregled API-ja

Collab Chat pruža tri REST API endpointa za programsko upravljanje tekstualnim razgovorima. Ovi endpointi omogućuju dohvaćanje, stvaranje i brisanje napomena bez korištenja widgeta u pregledniku.

Ovo su javni endpointi koji autentificiraju korisnike putem kolačića preglednika. Ne koriste API ključeve. Korisnici moraju biti prijavljeni u FastComments u svom pregledniku kako bi pristupili ovim endpointima.

### Osnovni URL

Svi Collab Chat API endpointi nalaze se pod:

[inline-code-attrs-start title = 'Osnovni URL'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/comment-collab-chats
[inline-code-end]

### Autentifikacija

Ovi endpointi autentificiraju korisnike putem kolačića preglednika. Ne koriste API ključeve. Korisnici moraju biti prijavljeni u FastComments u svom pregledniku kako bi pristupili ovim endpointima.

### Dohvati sve razgovore

Dohvaća sve tekstualne razgovore za određenu stranicu.

#### Endpoint

[inline-code-attrs-start title = 'GET Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET /comment-collab-chats/:tenantId?urlId=<urlId>
[inline-code-end]

#### Parametri

`tenantId` (parametar putanje, obavezan) je vaš FastComments Tenant ID.

`urlId` (query parametar, obavezan) je identifikator stranice za koju želite dohvatiti razgovore.

#### Odgovor

Odgovor uključuje status API-ja, informacije o trenutnoj korisničkoj sesiji ako je korisnik autentificiran, niz razgovora s njihovim ID-jevima, URL-ovima i tekstualnim rasponima, očišćeni URL identifikator, zastavicu koja označava je li trenutni korisnik administrator stranice ili moderator, te detalje WebSocket veze za sinkronizaciju u stvarnom vremenu uključujući `tenantIdWS`, `urlIdWS` i `userIdWS`.

#### Primjer zahtjeva

[inline-code-attrs-start title = 'Primjer GET zahtjeva'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl "https://fastcomments.com/comment-collab-chats/demo?urlId=my-article-page" \
  --cookie "your-session-cookie"
[inline-code-end]

#### Primjer odgovora

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

### Stvori razgovor

Stvara novi tekstualni razgovor za određeni odabir teksta.

#### Endpoint

[inline-code-attrs-start title = 'POST Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
POST /comment-collab-chats/:tenantId
[inline-code-end]

#### Parametri

`tenantId` (parametar putanje, obavezan) je vaš FastComments Tenant ID.

Tijelo zahtjeva mora biti JSON i mora uključivati ove obavezne polja.

`urlId` (string, obavezan) je osnovni identifikator stranice.

`urlIdWithRange` (string, obavezan) je URL kombiniran s tekstualnim rasponom, na primjer `my-page:p:0:15,0:45{abc123}`.

`pageTitle` (string, obavezan) je naslov stranice.

`selector` (string, obavezan) je DOM putanja do elementa koji sadrži odabrani tekst.

`range` (string, obavezan) je serializirani tekstualni raspon u formatu `startOffset:endOffset,startOffset:endOffset{checksum}`.

`createdFromCommentId` (string, obavezan) je ID komentara koji je pokrenuo ovaj razgovor.

`broadcastId` (string, obavezan) je UUID za sinkronizaciju u stvarnom vremenu radi sprječavanja efekata odjeka.

#### Odgovor

Odgovor uključuje status API-ja i ID novostvorenog razgovora.

#### Primjer zahtjeva

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

#### Primjer odgovora

[inline-code-attrs-start title = 'Primjer POST odgovora'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "createdChatId": "conv789"
}
[inline-code-end]

### Izbriši razgovor

Briše postojeći tekstualni razgovor. Ovaj endpoint zahtijeva administratorske ili moderatorske dozvole, ili razgovor mora biti stvorio autentificirani korisnik.

#### Endpoint

[inline-code-attrs-start title = 'DELETE Endpoint'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
DELETE /comment-collab-chats/:tenantId/:chatId
[inline-code-end]

#### Parametri

`tenantId` (parametar putanje, obavezan) je vaš FastComments Tenant ID.

`chatId` (parametar putanje, obavezan) je ID razgovora koji treba izbrisati.

`broadcastId` (tijelo zahtjeva, obavezan) je UUID za sinkronizaciju u stvarnom vremenu.

#### Primjer zahtjeva

[inline-code-attrs-start title = 'Primjer DELETE zahtjeva'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -X DELETE "https://fastcomments.com/comment-collab-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
[inline-code-end]

#### Primjer odgovora

[inline-code-attrs-start title = 'Primjer DELETE odgovora'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success"
}
[inline-code-end]

### Ograničenje brzine

Ovi endpointi podliježu standardnom ograničenju brzine FastComments API-ja. Middleware za ograničenje brzine primjenjuje se po najmoprimcu (tenant) kako bi se spriječila zloupotreba uz omogućavanje uobičajenih obrazaca korištenja.

### Odgovori o pogreškama

Svi endpointi vraćaju standardne HTTP statusne kodove. Odgovor 400 označava nevažeće parametre zahtjeva. Odgovor 401 znači da je autentifikacija nije uspjela. Odgovor 403 označava nedovoljne dozvole. Odgovor 404 znači da razgovor nije pronađen. Odgovor 429 označava prekoračenje ograničenja brzine.

Odgovori o pogreškama uključuju JSON tijelo s detaljima:

[inline-code-attrs-start title = 'Primjer odgovora o pogrešci'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "error",
  "message": "Conversation not found"
}
[inline-code-end]

### Praćenje korištenja

Stvaranje razgovora povećava vašu metriku korištenja `conversationCreateCount`. Sva aktivnost WebSocket sinkronizacije povećava `pubSubMessageCount` i `pubSubBandwidth`. Ove metrike možete pratiti u svojoj FastComments upravljačkoj ploči pod analitikom korištenja.