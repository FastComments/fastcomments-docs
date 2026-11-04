Spletne kljuke (webhooks) je mogoče upravljati tudi prek REST API. Na ta način se integracije, kot je Zapier, naročijo na dogodke komentarjev brez uporabe nadzorne plošče, in sledi vzorcu REST Hooks: naroči se, prejmi dogodke, odjavi se.

API-naročnine obstajajo poleg spletnih kljuk, konfiguriranih v nadzorni plošči. Dogodek komentarja se dostavi vsaki spletni kljuki, ki se ujema z njeno domeno, vsaka kot svoja dostava, ne glede na to, kako je bila spletna kljuka ustvarjena.

## Avtentikacija

Vsaka zahteva potrebuje vaš API ključ v glavi `x-api-key` (ali parameter poizvedbe `API_KEY`) in vaš ID najemnika v parametru poizvedbe `tenantId`. Oba sta prikazana na strani API Secret v nadzorni plošči.

## Naročnina

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
|-------|----------|-------------|
| `url` | Da | Absolutni http ali https URL. |
| `event` | Da | `comment-created`, `comment-updated` ali `comment-deleted`. |
| `domain` | Ne | Domena iz konfiguracije vašega računa. Privzeto `*`, ki prejema dogodke za vsako domeno. |
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

Če se isti URL znova naroči na isti dogodek in domeno, se vrne obstoječa naročnina namesto da bi ustvarila podvojeno, tako da lahko odjemalec varno poskusi znova. Vsak najemnik ima lahko do 50 API-naročnin.

## Seznam

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

Vrne vsako spletno kljuko za najemnika, vključno s tistimi, ki jih upravlja nadzorna plošča (`"source": "dashboard"`). Filtrirajte z `event`, `domain` ali `source`.

## Odjava

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

Brisanje naročnine tudi zavrže vse dogodke, ki so še v čakalni vrsti zanjo. Na ta način je mogoče izbrisati samo naročnine, ustvarjene prek API; spletna kljuka iz nadzorne plošče ali ID, ki ne obstaja na vašem računu, odgovori z `404` in kodo `not-found`. Spletne kljuke iz nadzorne plošče se urejajo na strani Webhooks.

## Tovorni podatki in podpisovanje

Dostave uporabljajo isti tovor kot spletne kljuke iz nadzorne plošče (glejte Data Structures) in so podpisane z isto shemo HMAC (glejte Security & API Tokens). API-naročnine nikoli ne prejmejo zastarele glave `token`, zato namesto tega preverite glavo `X-FastComments-Signature`.

## Vzorčni tovorni podatki

```
GET https://fastcomments.com/api/v1/webhooks/sample-payloads?tenantId=YOUR_TENANT_ID&event=comment-created&limit=3
```

Vrne najnovejše komentarje računa v točno isti obliki, kot jo nosi dostava, tako da lahko integracija prikaže resnične vzorčne podatke, preden prispe prvi dogodek. `event` je neobvezen in se samo preveri, saj vsak dogodek dostavi isti objekt komentarja. `limit` je privzeto 3 in sprejme vrednosti od 1 do 10. Stane 2 API kredita.

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

## Odgovarjanje s 410 Gone

Če končna točka API-naročnine odgovori s HTTP `410 Gone`, FastComments to obravnava kot odjavo: naročnina se izbriše skupaj s čakajočimi dogodki in nadaljnjih dostav ni več. Spletnih kljuk, konfiguriranih v nadzorni plošči, nikoli ne izbriše samodejno; zanje je 410 običajna napaka. Vsak drug status napake se poskusi znova in sčasoma onemogoči spletno kljuko, kot je opisano v How it Works & Handling Retries.

## Nadzorna plošča

API-naročnine se prikažejo na seznamu Webhooks z virom **API**, kjer jih skrbnik lahko uredi, onemogoči, ponovno omogoči ali izbriše.