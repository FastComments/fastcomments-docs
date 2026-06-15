Momenteel online kijkers van een pagina: personen waarvan de websocket-sessie op dit moment op de pagina is geabonneerd.
Geeft anonCount + totalCount terug (kamerbrede abonnees, inclusief anonieme kijkers die we niet opsommen).

## Parameters

| Naam | Type | Verplicht | Omschrijving |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| afterName | string | No |  |
| afterUserId | string | No |  |

## Antwoord

Geeft terug: [`GetOnlineUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOnlineUsers200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getOnlineUsers Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_14f9c3';
const urlId: string = 'article_20250615';
const afterName: string = 'marie.curie';
const afterUserId: string = 'u_92b7';
const result: GetOnlineUsers200Response = await getOnlineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---