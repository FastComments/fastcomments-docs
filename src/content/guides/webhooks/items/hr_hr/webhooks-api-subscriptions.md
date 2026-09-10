Webhooks također mogu biti upravljani putem REST API-ja. Tako integracije poput Zapiera pretplaćuju se na događaje komentara bez korištenja nadzorne ploče, a slijede uzorak REST Hooks: pretplata, primanje događaja, odjava.

API pretplate koegzistiraju uz webhookove konfigurirane u nadzornoj ploči. Događaj komentara se isporučuje webhooku nadzorne ploče za njegovu domenu i svakoj API pretplati koja se podudara, svaka kao zasebna isporuka. Ne postoji ograničenje na jednog pretplatnika po događaju.

## Autentikacija

Svaki zahtjev treba vaš API ključ u zaglavlju `x-api-key` (ili u parametru upita `API_KEY`) i vaš ID najemnika u parametru upita `tenantId`. Oba su prikazana na stranici API tajne u nadzornoj ploči.

## Pretplata

```
POST https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
x-api-key: YOUR_API_KEY
Content-Type: application/json

{
    "url": "https://hooks.zapier.com/hooks/catch/123/abc",
    "event": "comment-created"
}
```

| Polje | Obavezno | Opis |
|-------|----------|------|
| `url` | Da | Apsolutni http ili https URL. |
| `event` | Da | `comment-created`, `comment-updated` ili `comment-deleted`. |
| `domain` | Ne | Domenu iz konfiguracije vašeg računa. Zadano je `*`, što prima događaje za svaku domenu. |
| `method` | Ne | `POST` (zadano), `PUT` ili `DELETE`. |

Odgovor sadrži pretplatu:

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

Ponovna pretplata na isti URL za isti događaj i domenu vraća postojeću pretplatu umjesto stvaranja duplikata, pa klijent može sigurno ponoviti zahtjev. Svaki najemnik može imati najviše 50 API pretplata.

## Popis

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

Vraća sve webhookove za najemnika, uključujući one upravljane u nadzornoj ploči (`"source": "dashboard"`). Filtrirajte pomoću `event`, `domain` ili `source`.

## Odjava

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

Brisanje pretplate također odbacuje sve događaje koji su još u redu za nju. Samo pretplate kreirane putem API-ja mogu se izbrisati na ovaj način. Webhookovi nadzorne ploče se uređuju na stranici Webhooks.

## Tjelesni podaci i potpisivanje

Isporuke koriste isti tijelo podataka kao webhookovi nadzorne ploče (pogledajte Strukture podataka) i potpisane su istim HMAC shemom (pogledajte Sigurnost i API tokeni). API pretplate nikada ne primaju zastarjelo zaglavlje `token`, pa umjesto toga provjerite zaglavlje `X-FastComments-Signature`.

## Odgovor s 410 Gone

Ako krajnja točka API pretplate odgovori HTTP `410 Gone`, FastComments to tretira kao odjavu: pretplata se briše zajedno s događajima u redu, i ne poduzimaju se daljnje isporuke. Webhookovi konfigurirani u nadzornoj ploči se nikada ne brišu automatski; za njih je 410 obična greška. Svaki drugi status greške se ponavlja i na kraju onemogućuje webhook, kako je opisano u Kako funkcionira i Rukovanje ponovnim pokušajima.

## Nadzorna ploča

API pretplate su navedene na stranici Webhooks pod domenom za koju su kreirane, gdje administrator može onemogućiti, ponovo omogućiti ili ih izbrisati.