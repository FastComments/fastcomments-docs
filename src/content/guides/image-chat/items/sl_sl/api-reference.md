### Pregled API-ja

Image Chat ponuja tri REST API končne točke za programsko upravljanje slikovnih pogovorov. Te končne točke omogočajo pridobivanje, ustvarjanje in brisanje markerjev brez uporabe brskalniškega gradnika.

Vse API končne točke zahtevajo avtentikacijo in sledijo standardnim vzorcem FastComments API. To so javne končne točke, ki se avtentikirajo prek brskalniških piškotkov, ne prek API ključev.

### Osnovni URL

Vse Image Chat API končne točke so pod:

```
https://fastcomments.com/comment-image-chats
```

### Avtentikacija

Te končne točke avtentikirajo uporabnike prek brskalniških piškotkov. Ne uporabljajo API ključev. Uporabniki morajo biti v svojem brskalniku prijavljeni v FastComments, da dostopajo do teh končnih točk.

### Pridobivanje vseh pogovorov

Pridobite vse slikovne pogovore za določeno sliko.

#### Končna točka

```
GET /comment-image-chats/:tenantId?urlId=<urlId>
```

#### Parametri

`tenantId` (parameter poti, obvezen) je vaš FastComments Tenant ID.

`urlId` (parameter poizvedbe, obvezen) je identifikator slike, za katero želite pridobiti pogovore.

#### Odziv

Odziv vključuje status API, informacije o trenutni uporabniški seji, če je uporabnik avtentikiran, polje pogovorov z njihovimi ID-ji, URL-ji in koordinatami X/Y, očiščen URL identifikator, zastavico, ki označuje, ali je trenutni uporabnik skrbnik ali moderator spletnega mesta, ter podrobnosti o WebSocket povezavi za sinhronizacijo v živo, vključno s `tenantIdWS`, `urlIdWS` in `userIdWS`.

#### Primer zahteve

```bash
curl "https://fastcomments.com/comment-image-chats/demo?urlId=my-product-image" \
  --cookie "your-session-cookie"
```

#### Primer odziva

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

### Ustvarjanje pogovora

Ustvarite nov slikovni pogovor za določeno lokacijo na sliki.

#### Končna točka

```
POST /comment-image-chats/:tenantId
```

#### Parametri

`tenantId` (parameter poti, obvezen) je vaš FastComments Tenant ID.

Telo zahteve mora biti v formatu JSON in vključevati naslednja obvezna polja.

`urlId` (niz, obvezen) je osnovni identifikator strani.

`windowUrlId` (niz, obvezen) je URL, združen z virom slike in koordinatami, na primer `my-page:/images/photo.jpg:25.5:30.2`.

`pageTitle` (niz, obvezen) je naslov strani.

`src` (niz, obvezen) je URL vira slike.

`x` (številka, obvezna) je koordinata X kot odstotek (0-100).

`y` (številka, obvezna) je koordinata Y kot odstotek (0-100).

`createdFromCommentId` (niz, obvezen) je ID komentarja, ki je sprožil ta pogovor.

`broadcastId` (niz, obvezen) je UUID za sinhronizacijo v živo, da se preprečijo odmevni učinki.

#### Odziv

Odziv vključuje status API in ID novo ustvarjenega pogovora.

#### Primer zahteve

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

#### Primer odziva

```json
{
  "status": "success",
  "createdChatId": "conv789"
}
```

### Brisanje pogovora

Izbrišite obstoječi slikovni pogovor. Ta končna točka zahteva skrbniška ali moderatorska dovoljenja, ali pa mora biti pogovor ustvarjen s strani avtentikiranega uporabnika.

#### Končna točka

```
DELETE /comment-image-chats/:tenantId/:chatId
```

#### Parametri

`tenantId` (parameter poti, obvezen) je vaš FastComments Tenant ID.

`chatId` (parameter poti, obvezen) je ID pogovora za brisanje.

`broadcastId` (telo zahteve, obvezno) je UUID za sinhronizacijo v živo.

#### Primer zahteve

```bash
curl -X DELETE "https://fastcomments.com/comment-image-chats/demo/conv789" \
  --cookie "your-session-cookie" \
  -H "Content-Type: application/json" \
  -d '{
    "broadcastId": "550e8400-e29b-41d4-a716-446655440001"
  }'
```

#### Primer odziva

```json
{
  "status": "success"
}
```

### Sistem koordinat

Koordinati X in Y sta shranjeni kot odstotki dimenzij slike. X predstavlja vodoravni položaj od levega roba (0 = levi rob, 100 = desni rob). Y predstavlja navpični položaj od zgornjega roba (0 = zgornji rob, 100 = spodnji rob).

Te odstotne vrednosti lahko vključujejo decimalke za natančnost. Na primer, x: 25.5 pomeni 25,5 % od levega roba slike.

### Omejevanje hitrosti

Te končne točke so predmet standardnega omejevanja hitrosti FastComments API. Vmesna programska oprema za omejevanje hitrosti deluje na nivoju najemnika, da prepreči zlorabe, hkrati pa omogoča normalne vzorce uporabe.

### Odzivi napak

Vse končne točke vračajo standardne HTTP statusne kode. Odziv 400 označuje neveljavne parametre zahteve. Odziv 401 pomeni, da je avtentikacija spodletela. Odziv 403 označuje nezadostna dovoljenja. Odziv 404 pomeni, da pogovora ni bilo mogoče najti. Odziv 429 označuje prekoračitev omejitve hitrosti.

Odzivi napak vključujejo telo JSON s podrobnostmi:

```json
{
  "status": "error",
  "message": "Conversation not found"
}
```

### Sledenje uporabi

Ustvarjanje pogovorov poveča vašo metriko uporabe `conversationCreateCount`. Vska dejavnost WebSocket sinhronizacije poveča `pubSubMessageCount` in `pubSubBandwidth`. Te metrike lahko spremljate v nadzorni plošči FastComments pod analitiko uporabe.