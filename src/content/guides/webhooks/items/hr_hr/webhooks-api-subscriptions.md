Webhooks također mogu biti upravljani putem REST API‑ja. Tako integracije poput Zapiera pretplaćuju se na događaje komentara bez dodirivanja nadzorne ploče, a slijede uzorak REST Hooks: pretplata, primanje događaja, odjava.

API pretplate koegzistiraju uz webhooks konfigurirane u nadzornoj ploči. Događaj komentara isporučuje se svakom webhooku koji odgovara njegovoj domeni, svaki kao zasebna isporuka, bez obzira kako je webhook kreiran.

## Autentifikacija

Svaki zahtjev treba vaš API ključ u zaglavlju `x-api-key` (ili u parametru upita `API_KEY`) i
vaš ID najamnika u parametru upita `tenantId`. Oba su prikazana na stranici API tajne u nadzornoj ploči.

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
| `domain` | Ne | Domena iz konfiguracije vašeg računa. Zadano je `*`, što prima događaje za svaku domenu. |
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

Ponovna pretplata na isti URL za isti događaj i domenu vraća postojeću pretplatu umjesto stvaranja duplikata, pa klijent može sigurno ponoviti pokušaj. Svaki najamnik može imati najviše 50 API pretplata.

## Popis

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

Vraća svaki webhook za najamnika, uključujući one upravljane u nadzornoj ploči (`"source": "dashboard"`).
Filtrirajte pomoću `event`, `domain` ili `source`.

## Odjava

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

Brisanje pretplate također odbacuje sve događaje koji su još u redu za nju. Samo pretplate kreirane
preko API‑ja mogu se izbrisati na ovaj način; webhook iz nadzorne ploče ili ID koji ne postoji u vašem
računu odgovara s `404` i kodom `not-found`. Webhookovi iz nadzorne ploče uređuju se na stranici Webhooks.

## Učitci i potpisivanje

Isporuke koriste isti učitak kao webhookovi iz nadzorne ploče (pogledajte Strukture podataka) i potpisane su istim
HMAC shemom (pogledajte Sigurnost i API tokeni). API pretplate nikada ne primaju zastarjelo zaglavlje `token`, pa
provjerite zaglavlje `X-FastComments-Signature` umjesto toga.

## Primjeri učitaka

```
GET https://fastcomments.com/api/v1/webhooks/sample-payloads?tenantId=YOUR_TENANT_ID&event=comment-created&limit=3
```

Vraća najnovije komentare računa u točnom obliku koji isporuka nosi, tako da integracija može
prikazati stvarne primjere podataka prije nego prvi događaj stigne. `event` je opcionalan i samo se provjerava, budući da svaki
događaj isporučuje isti objekt komentara. `limit` je zadano 3 i prihvaća vrijednosti od 1 do 10. Troši 2 API kredita.

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

## Odgovor s 410 Gone

Ako krajnja točka API pretplate odgovori s HTTP `410 Gone`, FastComments to tretira kao
odjavu: pretplata se briše zajedno s događajima u redu, a daljnje isporuke se ne pokušavaju. Webhookovi konfigurirani u nadzornoj ploči nikada se ne brišu automatski; za njih je 410 obična greška. Svaki drugi status greške se ponavlja i na kraju onemogućuje webhook, kako je opisano u Kako funkcionira i rukovanje ponovnim pokušajima.

## Nadzorna ploča

API pretplate pojavljuju se u popisu Webhooks s izvorom **API**, gdje administrator može uređivati,
onemogućiti, ponovo omogućiti ili izbrisati ih.