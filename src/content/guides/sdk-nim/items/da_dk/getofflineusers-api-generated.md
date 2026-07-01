Past commenters on the page who are NOT currently online. Sorted by displayName.  
Brug dette efter at have udtømt /users/online for at gengive en "Members" sektion.  
Cursor-paginering på commenterName: serveren går igennem den delvise {tenantId, urlId, commenterName} indeks fra afterName fremad via $gt, ingen $skip-omkostning.

## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| options | GetOfflineUsersOptions | No |  |

## Respons

Returnerer: [`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

## Eksempel

[inline-code-attrs-start title = 'getOfflineUsers Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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