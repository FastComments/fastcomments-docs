Webhooks je mogoče upravljati tudi prek REST API-ja. Tako integracije, kot je Zapier, naročijo na dogodke komentarjev, ne da bi se dotaknile nadzorne plošče, in sledijo vzorcu REST Hooks: naročanje, prejemanje dogodkov, odjava.

Naročnine API-ja sobivajo z webhooki, ki so nastavljeni v nadzorni plošči. Dogodek komentarja se pošlje webhooku nadzorne plošče za njegovo domeno in vsakemu naročniku API-ja, ki se ujema, vsak kot ločeno dostavo. Ni omejitve enega naročnika na dogodek.

## Avtentikacija

Vsaka zahteva potrebuje vaš API ključ v glavi `x-api-key` (ali kot poizvedbeni parameter `API_KEY`) in vaš ID najemnika v poizvedbenem parametru `tenantId`. Obe vrednosti sta prikazani na strani API skrivnosti v nadzorni plošči.

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
| `domain` | Ne | Domena iz nastavitve vašega računa. Privzeto je `*`, ki prejme dogodke za vsako domeno. |
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

Ponovno naročanje istega URL‑ja na isti dogodek in domeno vrne obstoječo naročnino, namesto da bi ustvarilo podvojeno, zato lahko odjemalec varno ponovi zahtevo. Vsak najemnik lahko ima največ 50 naročnin API-ja.

## Seznam

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

Vrne vse webhooke za najemnika, vključno s tistimi, ki so upravljani v nadzorni plošči (`"source": "dashboard"`). Filtrirajte z `event`, `domain` ali `source`.

## Odjava

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

Brisanje naročnine tudi zavrže morebitne dogodke, ki so še v čakalni vrsti. Le naročnine, ustvarjene prek API-ja, je mogoče izbrisati na ta način. Webhooki nadzorne plošče se urejajo na strani Webhooks.

## Vsebina in podpisovanje

Dostave uporabljajo enako vsebino kot webhooki nadzorne plošče (glejte Strukture podatkov) in so podpisane z enakim HMAC shemom (glejte Varnost & API žetone). Naročnine API-ja nikoli ne prejmejo zastarele glave `token`, zato preverite glavo `X-FastComments-Signature`.

## Odgovor z 410 Gone

Če končna točka naročnine API-ja odgovori z HTTP `410 Gone`, FastComments to obravnava kot odjavo: naročnina se izbriše skupaj z njenimi čakajočimi dogodki, in nadaljnje dostave se ne poskušajo. Webhooki, nastavljeni v nadzorni plošči, se nikoli samodejno ne izbrišejo; za njih je 410 običajna napaka. Vsako drugo stanje napake se ponovi in sčasoma onemogoči webhook, kot je opisano v Kako deluje & Obvladovanje ponovnih poskusov.

## Nadzorna plošča

Naročnine API-ja so prikazane na strani Webhooks pod domeno, za katero so bile ustvarjene, kjer jih lahko skrbnik onemogoči, ponovno omogoči ali izbriše.