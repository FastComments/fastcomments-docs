---
Aktuell online Viewer einer Seite: Personen, deren Websocket‑Sitzung gerade die Seite abonniert.
Gibt anonCount + totalCount zurück (raumweite Abonnenten, einschließlich anonymer Viewer, die wir nicht aufzählen).

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|---------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| afterName | string | No |  |
| afterUserId | string | No |  |

## Antwort

Rückgabe: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOnlineResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getOnlineUsers Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchOnlineUsers() {
  const tenantId: string = "tenant_12345";
  const urlId: string = "article-9876";
  const afterName: string | undefined = "john_doe";
  const afterUserId: string | undefined = "user_456";

  const onlineUsers: PageUsersOnlineResponse = await getOnlineUsers(
    tenantId,
    urlId,
    afterName,
    afterUserId
  );

  console.log(onlineUsers);
}

fetchOnlineUsers();
[inline-code-end]

---