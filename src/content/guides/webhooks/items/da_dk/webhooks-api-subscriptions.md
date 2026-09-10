---
Webhooks kan også administreres via REST API'en. Sådan kan integrationer som Zapier abonnere på kommentarhændelser uden at røre ved instrumentbrættet, og det følger REST Hooks‑mønsteret: abonnere, modtage hændelser, afmelde.

API‑abonnementer lever side om side med de webhooks, der er konfigureret i instrumentbrættet. En kommentarhændelse leveres til hver webhook, der matcher dens domæne, hver som sin egen levering, uanset hvordan webhook'en blev oprettet.

## Godkendelse

Hver anmodning kræver din API‑nøgle i `x-api-key`‑headeren (eller `API_KEY`‑forespørgselsparameteren) og dit lejer‑ID i `tenantId`‑forespørgselsparameteren. Begge vises på siden API Secret i instrumentbrættet.

## Abonner

```
POST https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
x-api-key: YOUR_API_KEY
Content-Type: application/json

{
    "url": "https://hooks.zapier.com/hooks/catch/123/abc",
    "event": "comment-created"
}
```

| Felt | Påkrævet | Beskrivelse |
|------|----------|-------------|
| `url` | Ja | En absolut http- eller https-URL. |
| `event` | Ja | `comment-created`, `comment-updated` eller `comment-deleted`. |
| `domain` | Nej | Et domæne fra din kontokonfiguration. Standard er `*`, som modtager hændelser for alle domæner. |
| `method` | Nej | `POST` (standard), `PUT` eller `DELETE`. |

Svaret indeholder abonnementet:

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

At abonnere på den samme URL for den samme hændelse og domæne igen returnerer det eksisterende abonnement i stedet for at oprette en duplikat, så en klient trygt kan prøve igen. Hver lejer kan have op til 50 API‑abonnementer.

## Liste

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

Returnerer hver webhook for lejeren, inklusive dem, der administreres i instrumentbrættet (`"source": "dashboard"`). Filtrer med `event`, `domain` eller `source`.

## Afmeld

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

Sletning af et abonnement kasserer også eventuelle hændelser, der stadig er i kø for det. Kun abonnementer oprettet via API'en kan slettes på denne måde. Dashboard‑webhooks redigeres på siden Webhooks.

## Payloads og signering

Leveringer bruger den samme payload som dashboard‑webhooks (se Data Structures) og er signeret med det samme HMAC‑skema (se Security & API Tokens). API‑abonnementer modtager aldrig den ældre `token`‑header, så verificer i stedet `X-FastComments-Signature`‑headeren.

## Eksempelpayloads

```
GET https://fastcomments.com/api/v1/webhooks/sample-payloads?tenantId=YOUR_TENANT_ID&event=comment-created&limit=3
```

Returnerer kontoens seneste kommentarer i præcis den form, som en levering har, så en integration kan vise reelle eksempeldata inden den første hændelse ankommer. `event` er valgfri og kun valideret, da hver hændelse leverer det samme kommentarobjekt. `limit` er standard 3 og accepterer 1 til 10. Koster 2 API‑kreditter.

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

## Svar med 410 Gone

Hvis en API‑abonnements endpoint svarer med HTTP `410 Gone`, betragter FastComments det som en afmelding: abonnementet slettes sammen med dets køede hændelser, og der forsøges ikke flere leveringer. Webhooks, der er konfigureret i instrumentbrættet, slettes aldrig automatisk; for dem er en 410 en almindelig fejl. Alle andre fejlkoder forsøges igen og deaktiverer til sidst webhook'en, som beskrevet i How it Works & Handling Retries.

## Dashboard

API‑abonnementer vises i listen over Webhooks med kilden **API**, hvor en administrator kan redigere, deaktivere, genaktivere eller slette dem.

---