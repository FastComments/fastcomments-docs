### Pregled API-ja

Collab Chat pruža tri REST API krajnje točke za programsko upravljanje tekstualnim razgovorima. Ove krajnje točke omogućuju dohvaćanje, stvaranje i brisanje bilješki bez korištenja widgeta u pregledniku.

Ovo su javne krajnje točke koje provjeravaju korisnike putem kolačića preglednika. Ne koriste API ključeve. Korisnici moraju biti prijavljeni u FastComments u svom pregledniku da bi pristupili ovim krajnjim točkama.

### Base URL

Sve Collab Chat API krajnje točke nalaze se na:

[inline-code-attrs-start title = 'Osnovni URL'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/comment-collab-chats
[inline-code-end]

### Autentikacija

Ove krajnje točke provjeravaju korisnike putem kolačića preglednika. Ne koriste API ključeve. Korisnici moraju biti prijavljeni u FastComments u svom pregledniku da bi pristupili ovim krajnjim točkama.

### Dohvati sve razgovore

Dohvatite sve tekstualne razgovore za određenu stranicu.

#### Krajnja točka

[inline-code-attrs-start title = 'GET krajnja točka'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET /comment-collab-chats/:tenantId?urlId=<urlId>
[inline-code-end]

#### Parametri

`tenantId` (parametar putanje, obavezan) je vaš FastComments Tenant ID.

`urlId` (parametar upita, obavezan) je identifikator stranice za koju želite dohvatiti razgovore.

#### Odgovor

Odgovor uključuje status API-ja, informacije o trenutnoj korisničkoj sesiji ako je autentificiran, polje razgovora s njihovim ID-evima, URL-ovima i rasponima teksta, očišćeni identifikator URL-a, zastavicu koja pokazuje je li trenutni korisnik administrator stranice ili moderator, te detalje WebSocket veze za sinkronizaciju uživo koji uključuju `tenantIdWS`, `urlIdWS` i `userIdWS`.

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

### Kreiranje razgovora

Stvorite novi tekstualni razgovor za određeni odabir teksta.

#### Krajnja točka

[inline-code-attrs-start title = 'POST krajnja točka'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
POST /comment-collab-chats/:tenantId
[inline-code-end]

#### Parametri

`tenantId` (parametar putanje, obavezan) je vaš FastComments Tenant ID.

Tijelo zahtjeva mora biti JSON i sadržavati sljedeća obavezna polja.

`urlId` (string, obavezno) je osnovni identifikator stranice.

`urlIdWithRange` (string, obavezno) je URL kombiniran s rasponom teksta, na primjer `my-page:p:0:15,0:45{abc123}`.

`pageTitle` (string, obavezno) je naslov stranice.

`selector` (string, obavezno) je DOM putanja do elementa koji sadrži odabrani tekst.

`range` (string, obavezno) je serijalizirani raspon teksta u formatu `startOffset:endOffset,startOffset:endOffset{checksum}`.

`createdFromCommentId` (string, obavezno) je ID komentara koji je pokrenuo ovaj razgovor.

`broadcastId` (string, obavezno) je UUID za sinkronizaciju uživo kako bi se spriječili efekti odjeka.

#### Odgovor

Odgovor uključuje status API-ja i ID novo stvorenog razgovora.

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

### Brisanje razgovora

Obrišite postojeći tekstualni razgovor. Ova krajnja točka zahtijeva dozvole administratora ili moderatora, ili razgovor mora biti stvoren od strane prijavljenog korisnika.

#### Krajnja točka

[inline-code-attrs-start title = 'DELETE krajnja točka'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
DELETE /comment-collab-chats/:tenantId/:chatId
[inline-code-end]

#### Parametri

`tenantId` (parametar putanje, obavezan) je vaš FastComments Tenant ID.

`chatId` (parametar putanje, obavezan) je ID razgovora koji treba obrisati.

`broadcastId` (tijelo zahtjeva, obavezno) je UUID za sinkronizaciju uživo.

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

Ove krajnje točke podliježu standardnom ograničavanju brzine FastComments API-ja. Middleware za ograničenje brzine primjenjuje se po tenantu kako bi se spriječila zlouporaba, a istovremeno omogućili normalni obrasci korištenja.

### Odgovori s pogreškom

Sve krajnje točke vraćaju standardne HTTP statusne kodove. Odgovor 400 označava nevažeće parametre zahtjeva. Odgovor 401 znači da autentikacija nije uspjela. Odgovor 403 označava nedostatak dozvola. Odgovor 404 znači da razgovor nije pronađen. Odgovor 429 označava prekoračenje ograničenja stope.

Odgovori s pogreškom uključuju JSON tijelo s detaljima:

[inline-code-attrs-start title = 'Primjer odgovora s pogreškom'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "error",
  "message": "Conversation not found"
}
[inline-code-end]

### Praćenje upotrebe

Stvaranje razgovora povećava vašu metriku upotrebe `conversationCreateCount`. Sva aktivnost sinkronizacije putem WebSocket-a povećava `pubSubMessageCount` i `pubSubBandwidth`. Te metrike možete pratiti u vašoj FastComments nadzornoj ploči pod analizom upotrebe.

---