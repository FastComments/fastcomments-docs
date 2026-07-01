Momenteel online kijkers van een pagina: personen wiens websocketsessie op dit moment op de pagina geabonneerd is.  
Retourneert anonCount + totalCount (abonnees voor de hele ruimte, inclusief anonieme kijkers die we niet opsommen).

## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| afterName | string | Nee |  |
| afterUserId | string | Nee |  |

## Respons

Retourneert: [`GetOnlineUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOnlineUsersResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getOnlineUsers Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoOnlineUsers() {
  const tenantId: string = "tenant_12345";
  const urlId: string = "url_98765";

  // Met optionele paginatieparameters
  const pagedResult: GetOnlineUsersResponse = await getOnlineUsers(
    tenantId,
    urlId,
    "alice_smith",
    "user_9"
  );

  // Zonder optionele paginatieparameters
  const fullResult: GetOnlineUsersResponse = await getOnlineUsers(tenantId, urlId);
}
[inline-code-end]