Frühere Kommentatoren auf der Seite, die derzeit NICHT online sind. Sortiert nach displayName.
Verwenden Sie dies, nachdem /users/online erschöpft ist, um einen „Mitglieder"-Abschnitt darzustellen.
Cursor-Paginierung über commenterName: der Server durchläuft den partiellen {tenantId, urlId, commenterName}
Index von afterName vorwärts über $gt, ohne $skip-Kosten.

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| options | GetOfflineUsersOptions | Nein |  |

## Antwort

Gibt zurück: [`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

## Beispiel

[inline-code-attrs-start title = 'getOfflineUsers Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (offlineResp, httpResponse) = client.getOfflineUsers(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  options = GetOfflineUsersOptions()
)
if offlineResp.isSome:
  let offline = offlineResp.get()
  echo offline)
[inline-code-end]