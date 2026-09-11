Webhooks se takođe mogu upravljati putem REST API-ja. Ovo je način na koji integracije poput Zapiera pretplaćuju na događaje komentara bez korišćenja kontrolne table, i prati obrazac REST Hooks: pretplata, primanje događaja, otkazivanje pretplate.

API pretplate postoje uz webhooks konfigurirane u kontrolnoj tabli. Događaj komentara se isporučuje svakom webhooku koji odgovara njegovom domenu, svaki kao zasebna isporuka, bez obzira kako je webhook kreiran.

## Autentifikacija

Svaki zahtev zahteva vaš API ključ u zaglavlju `x-api-key` (ili u parametru upita `API_KEY`) i ID vašeg zakupca u parametru upita `tenantId`. Obe vrednosti su prikazane na stranici API tajne u kontrolnoj tabli.

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
| `url` | Yes | Apsolutni http ili https URL. |
| `event` | Yes | `comment-created`, `comment-updated` or `comment-deleted`. |
| `domain` | No | Domena iz konfiguracije vašeg naloga. Podrazumevano je `*`, što prima događaje za svaki domen. |
| `method` | No | `POST` (default), `PUT` or `DELETE`. |

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

Ponovno pretplata na isti URL za isti događaj i domen vraća postojeću pretplatu umesto kreiranja duplikata, tako da klijent može bezbedno ponoviti zahtev. Svaki zakupac može imati može do 50 API pretplata.

## Lista

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

Vraća sve webhookove za zakupca, uključujući i one upravljane u kontrolnoj tabli (`"source": "dashboard"`). Filtrirajte po `event`, `domain` ili `source`.

## Otkaži pretplatu

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

Brisanje pretplate takođe odbacuje sve događaje koji su još u redu za nju. Samo pretplate kreirane putem API-ja mogu se izbrisati na ovaj način; webhook iz kontrolne table, ili ID koji ne postoji na vašem nalogu, odgovara sa `404` i kodom `not-found`. Webhookovi iz kontrolne table se uređuju na stranici Webhooks.

## Payload-ovi i potpisivanje

Isporuke koriste isti payload kao webhookovi iz kontrolne table (pogledajte Data Structures) i potpisane su istim HMAC šemom (pogledajte Security & API Tokens). API pretplate nikada ne primaju zastarelo zaglavlje `token`, pa proverite zaglavlje `X-FastComments-Signature`.

## Primeri payload-ova

```
GET https://fastcomments.com/api/v1/webhooks/sample-payloads?tenantId=YOUR_TENANT_ID&event=comment-created&limit=3
```

Vraća najnovije komentare naloga u tačno onom obliku koji isporuka nosi, tako da integracija može prikazati stvarne uzorke podataka pre nego što prvi događaj stigne. `event` je opcionalan i samo se validira, pošto svaki događaj isporučuje isti objekat komentara. `limit` podrazumevano je 3 i prihvata vrednosti od 1 do 10. Troši 2 API kredita.

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

## Odgovor sa 410 Gone

Ako endpoint API pretplate odgovori HTTP `410 Gone`, FastComments to tretira kao otkazivanje pretplate: pretplata se briše zajedno sa svojim događajima u redu, i ne pokušavaju se dalje isporuke. Webhookovi konfigurirani u kontrolnoj tabli se nikada ne brišu automatski; za njih je 410 običan neuspeh. Svaki drugi status greške se ponovo pokušava i na kraju onemogućava webhook, kako je opisano u sekciji How it Works & Handling Retries.

## Kontrolna tabla

API pretplate se pojavljuju u listi Webhookova sa izvorom **API**, gde administrator može da ih uređuje, onemogući, ponovo omogući ili obriše.