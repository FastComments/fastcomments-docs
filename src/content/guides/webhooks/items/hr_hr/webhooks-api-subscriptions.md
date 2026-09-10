Webhooks također mogu biti upravljani putem REST API‑ja. Tako integracije poput Zapiera pretplaćuju
na događaje komentara bez korištenja nadzorne ploče, a slijede uzorak REST Hooks: pretplata,
primanje događaja, otkazivanje pretplate.

API pretplate koegzistiraju uz webhooks konfigurirane u nadzornoj ploči. Događaj komentara se isporučuje
svakom webhooku koji odgovara njegovoj domeni, svaki kao zasebna isporuka, neovisno o načinu na koji je webhook kreiran.

## Autentifikacija

Svaki zahtjev treba vaš API ključ u zaglavlju `x-api-key` (ili u parametru upita `API_KEY`) i
vaš ID najmodavca u parametru upita `tenantId`. Oba su prikazana na stranici API tajne u nadzornoj ploči.

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

Ponovno pretplaćivanje iste URL adrese na isti događaj i domenu vraća postojeću pretplatu umjesto
stvaranja duplikata, pa klijent može sigurno ponoviti pokušaj. Svaki najmodavac može imati najviše 50 API pretplata.

## Popis

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

Vraća svaki webhook za najmodavca, uključujući one upravljane u nadzornoj ploči (`"source": "dashboard"`).
Filtrirajte pomoću `event`, `domain` ili `source`.

## Otkaži pretplatu

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

Brisanje pretplate također odbacuje sve događaje koji su još u redu za nju. Samo pretplate kreirane
preko API‑ja mogu se izbrisati na ovaj način. Webhookovi iz nadzorne ploče uređuju se na stranici Webhooks.

## Tijela zahtjeva i potpisivanje

Isporuke koriste isto tijelo kao webhookovi iz nadzorne ploče (pogledajte Strukture podataka) i potpisane su istim
HMAC shemom (pogledajte Sigurnost i API tokeni). API pretplate nikada ne primaju zastarjelo zaglavlje `token`, pa
umjesto toga provjerite zaglavlje `X-FastComments-Signature`.

## Primjeri tijela

```
GET https://fastcomments.com/api/v1/webhooks/sample-payloads?tenantId=YOUR_TENANT_ID&event=comment-created&limit=3
```

Vraća najnovije komentare računa u točno onom obliku koji isporuka nosi, tako da integracija može
prikazati stvarne primjere podataka prije nego što prvi događaj stigne. `event` je opcionalan i samo se provjerava, budući da svaki
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

Ako krajnja točka API pretplate odgovori HTTP‑om `410 Gone`, FastComments to tretira kao
otkazivanje pretplate: pretplata se briše zajedno s događajima u redu, a daljnje isporuke se
nepokušavaju. Webhookovi konfigurirani u nadzornoj ploči nikada se ne brišu automatski; za njih je 410
obična greška. Svaki drugi status greške se ponavlja i na kraju onemogućuje webhook, kako je opisano
u Kako funkcionira i Obrada ponavljanja.

## Nadzorna ploča

API pretplate pojavljuju se u popisu Webhooks s izvorom **API**, gdje administrator može uređivati,
onemogućiti, ponovo omogućiti ili izbrisati ih.