Webhooks je mogoče upravljati tudi prek REST API-ja. Tako integracije, kot je Zapier, naročijo na dogodke komentarjev, ne da bi se dotaknile nadzorne plošče, in sledi vzorcu REST Hooks: naročanje, prejemanje dogodkov, odjavljanje.

API naročnine sobivajo z webhooki, ki so nastavljeni v nadzorni plošči. Dogodek komentarja se dostavi vsakemu webhooku, ki se ujema z njegovo domeno, vsak kot svojo dostavo, ne glede na to, kako je bil webhook ustvarjen.

## Avtentikacija

Vsaka zahteva potrebuje vaš API ključ v glavi `x-api-key` (ali v parametru poizvedbe `API_KEY`) in
vaš ID najemnika v parametru poizvedbe `tenantId`. Obe sta prikazani na strani API skrivnosti v nadzorni plošči.

## Naročanje

```
POST https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
x-api-key: YOUR_API_KEY
Content-Type: application/json

{
    "url": "https://hooks.zapier.com/hooks/catch/123/abc",
    "event": "comment-created"
}
```

| Polje | Obvezno | Opis |
|-------|----------|------|
| `url` | Da | Absolutni http ali https URL. |
| `event` | Da | `comment-created`, `comment-updated` ali `comment-deleted`. |
| `domain` | Ne | Domena iz konfiguracije vašega računa. Privzeto je `*`, kar prejme dogodke za vsako domeno. |
| `method` | Ne | `POST` (privzeto), `PUT` ali `DELETE`. |

Odgovor vsebuje naročnino:

```json
{
    "status": "success",
    "webhook": {
        "id": "66f1c4c1e7a2b3d4f5a6b7c8",
        "url": "https://hooks.zapier.com/hooks/catch/123/abc",
        "event": "comment-created",
        "domain": "*",
        "method": "POST",
        "source": "api",
        "enabled": true,
        "createdAt": "2026-09-08T12:00:00.000Z"
    }
}
```

Naročanje istega URL-ja na isti dogodek in domeno ponovno vrne obstoječo naročnino, namesto da bi ustvarila podvojeno, zato lahko odjemalec varno ponovi zahtevo. Vsak najemnik lahko ima največ 50 API naročnin.

## Seznam

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

Vrne vse webhooke za najemnika, vključno s tistimi, ki so upravljani v nadzorni plošči (`"source": "dashboard"`). Filtrirajte z `event`, `domain` ali `source`.

## Odjava

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

Brisanje naročnine tudi zavrže morebitne dogodke, ki so še v čakalni vrsti za njo. Le naročnine, ustvarjene prek API-ja, je mogoče izbrisati na ta način. Webhooki v nadzorni plošči se urejajo na strani Webhooks.

## Telesa sporočil in podpisovanje

Dostave uporabljajo enako telo sporočila kot webhooki v nadzorni plošči (glej Strukture podatkov) in so podpisane z enakim HMAC shemom (glej Varnost & API žetoni). API naročnine nikoli ne prejmejo zastarele glave `token`, zato preverite glavo `X-FastComments-Signature`.

## Vzorčna telesa sporočil

```
GET https://fastcomments.com/api/v1/webhooks/sample-payloads?tenantId=YOUR_TENANT_ID&event=comment-created&limit=3
```

Vrne najnovejše komentarje računa v točno isti obliki, kot jo nosi dostava, zato lahko integracija prikaže realne vzorčne podatke, preden prispe prvi dogodek. `event` je neobvezen in se le preveri, saj vsak dogodek dostavi isti objekt komentarja. `limit` privzeto je 3 in sprejme vrednosti od 1 do 10. Strošek je 2 API kredita.

```json
{
    "status": "success",
    "payloads": [
        {
            "id": "66f1c4c1e7a2b3d4f5a6b7c8",
            "urlId": "https://example.com/blog/hello-world",
            "commenterName": "Jane Reader",
            "comment": "Great article!",
            "date": "2026-09-08T12:00:00.000Z",
            "approved": true
        }
    ]
}
```

## Odzivanje z 410 Gone

Če končna točka API naročnine odgovori z HTTP `410 Gone`, FastComments to obravnava kot odjavo: naročnina se izbriše skupaj z njenimi čakajočimi dogodki, in nadaljnje dostave se ne poskušajo. Webhooki, nastavljeni v nadzorni plošči, se nikoli samodejno ne izbrišejo; za njih je 410 običajna napaka. Vsako drugo stanje napake se ponovi in sčasoma onemogoči webhook, kot je opisano v Kako deluje & Obvladovanje ponovnih poskusov.

## Nadzorna plošča

API naročnine se prikažejo na seznamu Webhooks z virom **API**, kjer jih lahko skrbnik ureja, onemogoči, ponovno omogoči ali izbriše.