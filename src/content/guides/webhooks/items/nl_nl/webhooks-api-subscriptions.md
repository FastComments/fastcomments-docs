Webhooks kunnen ook worden beheerd via de REST API. Dit is hoe integraties zoals Zapier zich abonneren op commentaar‑gebeurtenissen zonder het dashboard aan te raken, en het volgt het REST Hooks‑patroon: abonneren, gebeurtenissen ontvangen, afmelden.

API-abonnementen bestaan naast de webhooks die in het dashboard zijn geconfigureerd. Een commentaar‑gebeurtenis wordt afgeleverd bij de dashboard‑webhook voor zijn domein en bij elk API‑abonnement dat overeenkomt, elk als een eigen levering. Er is geen limiet van één abonnee per gebeurtenis.

## Authentication

Elke aanvraag heeft uw API‑sleutel nodig in de `x-api-key` header (of de `API_KEY` query‑parameter) en uw tenant‑ID in de `tenantId` query‑parameter. Beide worden weergegeven op de API‑Secret‑pagina in het dashboard.

## Subscribe

```
POST https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
x-api-key: YOUR_API_KEY
Content-Type: application/json

{
    "url": "https://hooks.zapier.com/hooks/catch/123/abc",
    "event": "comment-created"
}
```

| Veld   | Verplicht | Beschrijving |
|--------|-----------|--------------|
| `url`  | Ja        | Een absolute http- of https-URL. |
| `event`| Ja        | `comment-created`, `comment-updated` of `comment-deleted`. |
| `domain`| Nee      | Een domein uit uw accountconfiguratie. Standaard `*`, wat gebeurtenissen voor elk domein ontvangt. |
| `method`| Nee      | `POST` (standaard), `PUT` of `DELETE`. |

De respons bevat het abonnement:

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

Het abonneren van dezelfde URL op dezelfde gebeurtenis en hetzelfde domein retourneert opnieuw het bestaande abonnement in plaats van een duplicaat te maken, zodat een client veilig kan opnieuw proberen. Elke tenant kan tot 50 API‑abonnementen hebben.

## List

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

Retourneert elke webhook voor de tenant, inclusief die beheerd in het dashboard (`"source": "dashboard"`). Filter met `event`, `domain` of `source`.

## Unsubscribe

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

Het verwijderen van een abonnement verwijdert ook alle nog in de wachtrij staande gebeurtenissen. Alleen abonnementen die via de API zijn aangemaakt, kunnen op deze manier worden verwijderd. Dashboard‑webhooks worden bewerkt op de Webhooks‑pagina.

## Payloads and signing

Leveringen gebruiken dezelfde payload als dashboard‑webhooks (zie Data Structures) en worden ondertekend met hetzelfde HMAC‑schema (zie Security & API Tokens). API‑abonnementen ontvangen nooit de verouderde `token` header, dus controleer in plaats daarvan de `X-FastComments-Signature` header.

## Responding with 410 Gone

Als het eindpunt van een API‑abonnement reageert met HTTP `410 Gone`, beschouwt FastComments dit als een afmelding: het abonnement wordt verwijderd samen met de in de wachtrij staande gebeurtenissen, en er worden geen verdere leveringen geprobeerd. Webhooks die in het dashboard zijn geconfigureerd, worden nooit automatisch verwijderd; voor hen is een 410 een gewone fout. Elke andere foutstatus wordt opnieuw geprobeerd en schakelt uiteindelijk de webhook uit, zoals beschreven in How it Works & Handling Retries.

## Dashboard

API‑abonnementen worden weergegeven op de Webhooks‑pagina onder het domein waarvoor ze zijn aangemaakt, waar een beheerder ze kan uitschakelen, opnieuw inschakelen of verwijderen.