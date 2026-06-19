Momenteel online kijkers van een pagina: personen wiens websocket-sessie op dit moment op de pagina is geabonneerd.
Geeft anonCount + totalCount terug (abonnees voor de hele ruimte, inclusief anonieme kijkers die we niet opsommen).

## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| afterName | string | Nee |  |
| afterUserId | string | Nee |  |

## Antwoord

Geeft terug: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOnlineResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getOnlineUsers Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f3c2b7';
const urlId: string = 'article-2026-06-19-site-update';
const afterName: string = 'michael.hansen';
const afterUserId: string = 'user_00421';
const onlineUsers: PageUsersOnlineResponse = await getOnlineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---