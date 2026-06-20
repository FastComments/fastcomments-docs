---
Frühere Kommentierende auf der Seite, die momentan NICHT online sind. Sortiert nach displayName.
Verwenden Sie dies, nachdem Sie /users/online abgefragt haben, um einen "Members"-Abschnitt anzuzeigen.
Cursor-Paginierung auf commenterName: Der Server durchläuft den partiellen Index {tenantId, urlId, commenterName}
index ab afterName vorwärts mittels $gt, ohne $skip-Kosten.

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| afterName | string | Nein |  |
| afterUserId | string | Nein |  |

## Antwort

Gibt zurück: [`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

## Beispiel

[inline-code-attrs-start title = 'getOfflineUsers Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getOfflineUsers(
  tenantId = "my-tenant-123",
  urlId = "news/article-how-to-code",
  afterName = "",
  afterUserId = ""
)

if response.isSome:
  let offlinePage = response.get()
  echo "Received offline users page"
  discard httpResponse.statusCode
[inline-code-end]

---