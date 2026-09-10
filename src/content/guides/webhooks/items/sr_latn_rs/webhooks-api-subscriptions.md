Webhooks se takođe mogu upravljati putem REST API‑a. Ovo je način na koji integracije poput Zapiera pretplaćuju se na događaje komentara bez korišćenja kontrolne table, i prati obrazac REST Hooks: pretplata, primanje događaja, otkazivanje pretplate.

API pretplate postoje uz webhooks‑ove konfigurisane u kontrolnoj tabli. Događaj komentara se isporučuje svakom webhook‑u koji odgovara njegovom domenu, svaki kao zasebna isporuka, bez obzira kako je webhook kreiran.

## Autentifikacija

Svaki zahtev zahteva vaš API ključ u zaglavlju `x-api-key` (ili parametar upita `API_KEY`) i
vaš tenant ID u parametru upita `tenantId`. Obe vrednosti su prikazane na stranici API Secret u kontrolnoj tabli.

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
| `domain` | Ne | Domen iz konfiguracije vašeg naloga. Podrazumevano je `*`, što prima događaje za svaki domen. |
| `method` | Ne | `POST` (podrazumevano), `PUT` ili `DELETE`. |

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

Ponovna pretplata na isti URL za isti događaj i domen vraća postojeću pretplatu umesto kreiranja duplikata, tako da klijent može bezbedno ponoviti zahtev. Svaki zakupac može imati najviše 50 API pretplata.

## Lista

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

Vraća sve webhook‑ove za zakupca, uključujući one upravljane u kontrolnoj tabli (`"source": "dashboard"`).  
Filtrirajte po `event`, `domain` ili `source`.

## Otkaži pretplatu

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

Brisanje pretplate takođe odbacuje sve događaje koji su još u redu za nju. Samo pretplate kreirane putem API‑a mogu se na ovaj način izbrisati. Webhook‑ovi iz kontrolne table se uređuju na stranici Webhooks.

## Payload-ovi i potpisivanje

Isporuke koriste isti payload kao webhook‑ovi iz kontrolne table (vidi Strukture podataka) i potpisane su istim HMAC šemom (vidi Bezbednost i API tokeni). API pretplate nikada ne primaju zastareli `token` zaglavlje, pa proverite `X-FastComments-Signature` zaglavlje.

## Primer payload-ova

```
GET https://fastcomments.com/api/v1/webhooks/sample-payloads?tenantId=YOUR_TENANT_ID&event=comment-created&limit=3
```

Vraća najnovije komentare naloga tačno u obliku koji isporuka nosi, tako da integracija može prikazati stvarne uzorke podataka pre nego što prvi događaj stigne. `event` je opcionalan i samo se validira, pošto svaki događaj isporučuje isti objekat komentara. `limit` podrazumevano je 3 i prihvata vrednosti od 1 do 10. Troši 2 API kredita.

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

Ako endpoint API pretplate odgovori HTTP `410 Gone`, FastComments to tretira kao otkazivanje pretplate: pretplata se briše zajedno sa svojim događajima u redu, i ne pokušavaju se dalje isporuke. Webhook‑ovi konfigurisani u kontrolnoj tabli se nikada ne brišu automatski; za njih je 410 običan neuspeh. Svaki drugi status greške se ponovo pokušava i na kraju onemogućava webhook, kako je opisano u Kako funkcioniše & Obrada ponovnih pokušaja.

## Kontrolna tabla

API pretplate se pojavljuju u listi Webhooks sa izvorom **API**, gde administrator može da ih uređuje, onemogući, ponovo omogući ili obriše.