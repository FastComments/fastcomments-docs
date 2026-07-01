Past commenters on the page who are NOT currently online. Sorted by displayName.  
Frühere Kommentatoren auf der Seite, die **nicht** gerade online sind. Sortiert nach Anzeigename.  

Use this after exhausting /users/online to render a "Members" section.  
Verwenden Sie dies, nachdem /users/online erschöpft ist, um einen „Mitglieder“-Abschnitt zu rendern.  

Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.  
Cursor-Paginierung bei commenterName: Der Server durchläuft den Teil {tenantId, urlId, commenterName} vom Index nach afterName vorwärts mittels $gt, ohne $skip-Kosten.  

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| options | GetOfflineUsersOptions | No |  |

## Response

Rückgabe: [`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

## Example

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