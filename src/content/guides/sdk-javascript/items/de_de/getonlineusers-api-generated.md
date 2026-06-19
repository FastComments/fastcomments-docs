Derzeit online befindliche Betrachter einer Seite: Personen, deren websocket-Sitzung gerade auf die Seite abonniert ist.
Gibt anonCount + totalCount zurück (raumweite Abonnenten, einschließlich anon-Betrachter, die wir nicht aufzählen).

## Parameter

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| afterName | string | Nein |  |
| afterUserId | string | Nein |  |

## Antwort

Gibt zurück: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOnlineResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'Beispiel: getOnlineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f3c2b7';
const urlId: string = 'article-2026-06-19-site-update';
const afterName: string = 'michael.hansen';
const afterUserId: string = 'user_00421';
const onlineUsers: PageUsersOnlineResponse = await getOnlineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---