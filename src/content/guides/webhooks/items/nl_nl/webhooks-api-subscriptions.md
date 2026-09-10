Webhooks kunnen ook worden beheerd via de REST API. Dit is hoe integraties zoals Zapier zich abonneren op commentaar‑gebeurtenissen zonder het dashboard aan te raken, en het volgt het REST Hooks‑patroon: abonneren, gebeurtenissen ontvangen, afmelden.

API-abonnementen bestaan naast de webhooks die in het dashboard zijn geconfigureerd. Een commentaar‑gebeurtenis wordt afgeleverd aan elke webhook die overeenkomt met zijn domein, elk als een eigen levering, ongeacht hoe de webhook is aangemaakt.

## Authenticatie

Elke aanvraag heeft uw API‑sleutel nodig in de `x-api-key` header (of de `API_KEY` query‑parameter) en uw tenant‑ID in de `tenantId` query‑parameter. Beide worden weergegeven op de API‑Secret‑pagina in het dashboard.

## Abonneren

```
POST https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
x-api-key: YOUR_API_KEY
Content-Type: application/json

{
    "url": "https://hooks.zapier.com/hooks/catch/123/abc",
    "event": "comment-created"
}
```

| Veld | Verplicht | Beschrijving |
|------|-----------|--------------|
| `url` | Ja | Een absolute http- of https‑URL. |
| `event` | Ja | `comment-created`, `comment-updated` of `comment-deleted`. |
| `domain` | Nee | Een domein uit uw accountconfiguratie. Standaard `*`, wat gebeurtenissen voor elk domein ontvangt. |
| `method` | Nee | `POST` (standaard), `PUT` of `DELETE`. |

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

## Lijst

```
GET https://fastcomments.com/api/v1/webhooks?tenantId=YOUR_TENANT_ID
```

Retourneert elke webhook voor de tenant, inclusief die beheerd in het dashboard (`"source": "dashboard"`). Filter met `event`, `domain` of `source`.

## Afmelden

```
DELETE https://fastcomments.com/api/v1/webhooks/SUBSCRIPTION_ID?tenantId=YOUR_TENANT_ID
```

Het verwijderen van een abonnement verwijdert ook alle nog in de wachtrij staande gebeurtenissen. Alleen abonnementen die via de API zijn aangemaakt, kunnen op deze manier worden verwijderd. Dashboard‑webhooks worden bewerkt op de Webhooks‑pagina.

## Payloads en ondertekening

Leveringen gebruiken dezelfde payload als dashboard‑webhooks (zie Data‑Structuren) en worden ondertekend met hetzelfde HMAC‑schema (zie Beveiliging & API‑Tokens). API‑abonnementen ontvangen nooit de verouderde `token` header, dus verifieer in plaats daarvan de `X-FastComments-Signature` header.

## Voorbeeld‑payloads

```
GET https://fastcomments.com/api/v1/webhooks/sample-payloads?tenantId=YOUR_TENANT_ID&event=comment-created&limit=3
```

Retourneert de meest recente commentaren van het account in precies de vorm die een levering heeft, zodat een integratie echte voorbeeldgegevens kan tonen voordat de eerste gebeurtenis arriveert. `event` is optioneel en alleen gevalideerd, aangezien elke gebeurtenis hetzelfde commentaarobject levert. `limit` standaard op 3 en accepteert 1 tot 10. Kost 2 API‑credits.

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

## Reageren met 410 Gone

Als het eindpunt van een API‑abonnement reageert met HTTP `410 Gone`, beschouwt FastComments dat als een afmelding: het abonnement wordt verwijderd samen met de in de wachtrij staande gebeurtenissen, en er worden geen verdere leveringen geprobeerd. Webhooks die in het dashboard zijn geconfigureerd, worden nooit automatisch verwijderd; voor hen is een 410 een gewone fout. Elke andere foutstatus wordt opnieuw geprobeerd en schakelt uiteindelijk de webhook uit, zoals beschreven in Hoe het werkt & Het afhandelen van retries.

## Dashboard

API‑abonnementen verschijnen in de Webhooks‑lijst met de bron **API**, waar een beheerder ze kan bewerken, uitschakelen, opnieuw inschakelen of verwijderen.

---