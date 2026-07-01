Currently-online viewers of a page: Personen, deren Websocket‑Sitzung gerade die Seite abonniert hat.  
Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).  

## Parameters

| Name | Typ | Erforderlich | Beschreibung |
|------|-----|--------------|--------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| afterName | string | Nein |  |
| afterUserId | string | Nein |  |

## Antwort

Returns: [`GetOnlineUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOnlineUsersResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getOnlineUsers Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoOnlineUsers() {
  const tenantId: string = "tenant_12345";
  const urlId: string = "url_98765";

  // With optional pagination parameters
  const pagedResult: GetOnlineUsersResponse = await getOnlineUsers(
    tenantId,
    urlId,
    "alice_smith",
    "user_9"
  );

  // Without optional pagination parameters
  const fullResult: GetOnlineUsersResponse = await getOnlineUsers(tenantId, urlId);
}
[inline-code-end]