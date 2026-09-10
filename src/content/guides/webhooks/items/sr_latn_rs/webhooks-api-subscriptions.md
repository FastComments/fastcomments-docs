Webhooks se takođe mogu upravljati putem REST API-ja. Ovo je način na koji integracije poput Zapiera pretplaćuju na događaje komentara bez dodirivanja kontrolne table, i prati obrazac REST Hooks: pretplata, primanje događaja, otkazivanje pretplate.

API pretplate žive uz webhookove konfigurirane u kontrolnoj tabli. Događaj komentara se isporučuje webhooku kontrolne table za njegov domen i svakoj API pretplati koja se podudara, svaka kao svoj isporuka. Nema ograničenja od jednog pretplatnika po događaju.

## Autentifikacija

Svaki zahtev zahteva vaš API ključ u zaglavlju `x-api-key` (ili parametar upita `API_KEY`) i vaš tenant ID u parametru upita `tenantId`. Oba su prikazana na stranici API tajne u kontrolnoj tabli.

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
| `domain` | Ne | Domena iz konfiguracije vašeg naloga. Podrazumevano je `*`, što prima događaje za svaki domen. |
| `method` | Ne | `POST` (default), `PUT` ili `DELETE`. |

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

Pretplata na isti URL za isti događaj i domen ponovo vraća postojeću pretplatu umesto kreiranja duplikata, tako da klijent može bezbedno ponoviti zahtev. Svaki tenant može imati najviše 50 API pretplata.

## Lista

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

Vraća svaki webhook za tenant, uključujući one upravljane u kontrolnoj tabli (`"source": "dashboard"`). Filtrirajte po `event`, `domain` ili `source`.

## Otkazivanje pretplate

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

Brisanje pretplate takođe odbacuje sve događaje koji su još u redu za nju. Samo pretplate kreirane putem API-ja mogu se izbrisati na ovaj način. Webhookovi kontrolne table se uređuju na stranici Webhooks.

## Payload-ovi i potpisivanje

Isporuke koriste isti payload kao webhookovi kontrolne table (pogledajte Strukture podataka) i potpisane su istim HMAC šemom (pogledajte Bezbednost & API tokeni). API pretplate nikada ne primaju zastarelo zaglavlje `token`, pa proverite zaglavlje `X-FastComments-Signature`.

## Odgovor sa 410 Gone

Ako endpoint API pretplate odgovori HTTP `410 Gone`, FastComments to tretira kao otkazivanje pretplate: pretplata se briše zajedno sa svojim događajima u redu, i dalje isporuke se ne pokušavaju. Webhookovi konfigurirani u kontrolnoj tabli se nikada ne brišu automatski; za njih je 410 običan neuspeh. Svaki drugi status greške se ponovo pokušava i na kraju onemogućava webhook, kako je opisano u Kako funkcioniše & Rukovanje ponovnim pokušajima.

## Kontrolna tabla

API pretplate su navedene na stranici Webhooks ispod domena za koji su kreirane, gde administrator može onemogućiti, ponovo omogućiti ili izbrisati ih.