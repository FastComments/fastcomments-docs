Webhooks kan også administreres via REST API'et. Sådan kan integrationer som Zapier abonnere på kommentarbegivenheder uden at røre ved dashboardet, og det følger REST Hooks‑mønsteret: abonnere, modtage begivenheder, afmelde.

API‑abonnementer lever side om side med de webhooks, der er konfigureret i dashboardet. En kommentarbegivenhed leveres til dashboardets webhook for dens domæne og til hvert API‑abonnement, der matcher, hver som sin egen levering. Der er ingen grænse på én abonnent pr. begivenhed.

## Godkendelse

Hver anmodning kræver din API‑nøgle i `x-api-key`‑headeren (eller `API_KEY`‑forespørgselsparameteren) og dit lejer‑ID i `tenantId`‑forespørgselsparameteren. Begge vises på siden API Secret i dashboardet.

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
| `domain` | Nej | Et domæne fra din kontokonfiguration. Standard er `*`, som modtager begivenheder for alle domæner. |
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

At abonnere på den samme URL for den samme begivenhed og domæne igen returnerer det eksisterende abonnement i stedet for at oprette en duplikat, så en klient kan trygt prøve igen. Hver lejer kan have op til 50 API‑abonnementer.

## Liste

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

Returnerer hver webhook for lejeren, inklusive dem, der administreres i dashboardet (`"source": "dashboard"`). Filtrer med `event`, `domain` eller `source`.

## Afmeld

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

Sletning af et abonnement kasserer også eventuelle begivenheder, der stadig er i kø for det. Kun abonnementer oprettet via API'et kan slettes på denne måde. Dashboard‑webhooks redigeres på siden Webhooks.

## Payloads og signering

Leveringer bruger den samme payload som dashboard‑webhooks (se Data Structures) og er signeret med det samme HMAC‑skema (se Security & API Tokens). API‑abonnementer modtager aldrig den ældre `token`‑header, så verificer i stedet `X-FastComments-Signature`‑headeren.

## Svar med 410 Gone

Hvis en API‑abonnements endpoint svarer med HTTP `410 Gone`, betragter FastComments det som en afmelding: abonnementet slettes sammen med dets køede begivenheder, og der foretages ingen yderligere leveringer. Webhooks, der er konfigureret i dashboardet, slettes aldrig automatisk; for dem er en 410 en almindelig fejl. Enhver anden fejlkode forsøges igen og deaktiverer til sidst webhooken, som beskrevet i How it Works & Handling Retries.

## Dashboard

API‑abonnementer er listet på siden Webhooks under det domæne, de blev oprettet for, hvor en administrator kan deaktivere, genaktivere eller slette dem.