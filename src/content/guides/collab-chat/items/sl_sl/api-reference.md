### Pregled API-ja

Collab Chat zagotavlja tri REST API končne točke za programsko upravljanje besedilnih pogovorov. Te končne točke vam omogočajo pridobivanje, ustvarjanje in brisanje opomb brez uporabe vtičnika v brskalniku.

Gre za javne končne točke, ki avtenticirajo uporabnike prek piškotkov brskalnika. Ne uporabljajo API ključev. Uporabniki morajo biti v svojem brskalniku prijavljeni v FastComments, da lahko dostopajo do teh končnih točk.

### Osnovni URL

Vse API končne točke Collab Chat so pod:

[inline-code-attrs-start title = 'Osnovni URL'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/comment-collab-chats
[inline-code-end]

### Avtentikacija

Te končne točke avtenticirajo uporabnike prek piškotkov brskalnika. Ne uporabljajo API ključev. Uporabniki morajo biti v svojem brskalniku prijavljeni v FastComments, da lahko dostopajo do teh končnih točk.

### Pridobi vse pogovore

Pridobi vse besedilne pogovore za določeno stran.

#### Končna točka

[inline-code-attrs-start title = 'GET končna točka'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
GET /comment-collab-chats/:tenantId?urlId=<urlId>
[inline-code-end]

#### Parametri

`tenantId` (parameter poti, obvezno) je vaš FastComments Tenant ID.

`urlId` (parameter poizvedbe, obvezno) je identifikator strani, za katero želite pridobiti pogovore.

#### Odgovor

Odgovor vključuje stanje API-ja, informacije o trenutni uporabniški seji, če je avtenticiran, polje pogovorov z njihovimi ID-ji, URL-ji in besedilnimi razponi, očiščen identifikator URL, zastavico, ki označuje, ali je trenutni uporabnik skrbnik strani ali moderator, in podrobnosti o WebSocket povezavi za sinhronizacijo v živo, vključno z `tenantIdWS`, `urlIdWS` in `userIdWS`.

#### GET - primer zahteve

[inline-code-attrs-start title = 'GET - primer zahteve'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl "https://fastcomments.com/comment-collab-chats/demo?urlId=my-article-page" \
  --cookie "your-session-cookie"
[inline-code-end]

#### GET - primer odgovora

[inline-code-attrs-start title = 'GET - primer odgovora'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

### Ustvari pogovor

Ustvari nov besedilni pogovor za izbrano besedilo.

#### Končna točka

[inline-code-attrs-start title = 'POST končna točka'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
POST /comment-collab-chats/:tenantId
[inline-code-end]

#### Parametri

`tenantId` (parameter poti, obvezno) je vaš FastComments Tenant ID.

Telo zahteve mora biti v formatu JSON in mora vsebovati naslednja obvezna polja.

`urlId` (string, obvezno) je osnovni identifikator strani.

`urlIdWithRange` (string, obvezno) je URL v kombinaciji z besedilnim razponom, na primer `my-page:p:0:15,0:45{abc123}`.

`pageTitle` (string, obvezno) je naslov strani.

`selector` (string, obvezno) je DOM pot do elementa, ki vsebuje izbrano besedilo.

`range` (string, obvezno) je serializiran besedilni razpon v formatu `startOffset:endOffset,startOffset:endOffset{checksum}`.

`createdFromCommentId` (string, obvezno) je ID komentarja, ki je sprožil ta chat.

`broadcastId` (string, obvezno) je UUID za sinhronizacijo v živo in preprečevanje odmevnih učinkov.

#### Odgovor

Odgovor vključuje stanje API-ja in ID novo ustvarjenega pogovora.

#### POST - primer zahteve

[inline-code-attrs-start title = 'POST - primer zahteve'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
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

#### POST - primer odgovora

[inline-code-attrs-start title = 'POST - primer odgovora'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success",
  "createdChatId": "conv789"
}
[inline-code-end]

### Izbriši pogovor

Izbriši obstoječ besedilni pogovor. Ta končna točka zahteva skrbniške ali moderatorske pravice, ali pa mora biti pogovor ustvaril overjeni uporabnik.

#### Končna točka

[inline-code-attrs-start title = 'DELETE končna točka'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
DELETE /comment-collab-chats/:tenantId/:chatId
[inline-code-end]

#### Parametri

`tenantId` (parameter poti, obvezno) je vaš FastComments Tenant ID.

`chatId` (parameter poti, obvezno) je ID pogovora, ki ga želite izbrisati.

`broadcastId` (telo zahteve, obvezno) je UUID za sinhronizacijo v živo.

#### DELETE - primer zahteve

[inline-code-attrs-start title = 'DELETE - primer zahteve'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -X DELETE "https://fastcomments.com/comment-collab-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
[inline-code-end]

#### DELETE - primer odgovora

[inline-code-attrs-start title = 'DELETE - primer odgovora'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success"
}
[inline-code-end]

### Omejevanje števila zahtev

Te končne točke so predmet standardnega omejevanja števila zahtev API-ja FastComments. Vmesnik za omejevanje zahtev deluje na ravni najemnika, da prepreči zlorabe ob hkratnem dovoljevanju običajnih vzorcev uporabe.

### Odzivi z napakami

Vse končne točke vračajo standardne HTTP statusne kode. Odziv 400 označuje neveljavne parametre zahteve. Odziv 401 pomeni, da avtentikacija ni uspela. Odziv 403 označuje neustrezna dovoljenja. Odziv 404 pomeni, da pogovor ni bil najden. Odziv 429 pomeni preseženo omejitev zahtev.

Odzivi z napako vključujejo JSON telo s podrobnostmi:

[inline-code-attrs-start title = 'Primer odgovora z napako'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "error",
  "message": "Conversation not found"
}
[inline-code-end]

### Sledenje uporabe

Ustvarjanje pogovorov poveča vašo metriko uporabe `conversationCreateCount`. Vsa dejavnost sinhronizacije prek WebSocket poveča `pubSubMessageCount` in `pubSubBandwidth`. Te metrike lahko spremljate v nadzorni plošči FastComments pod analitiko uporabe.

---