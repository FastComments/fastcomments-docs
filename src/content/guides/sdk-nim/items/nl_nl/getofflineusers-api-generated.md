---
Vorige commenters op de pagina die NIET momenteel online zijn. Gesorteerd op **displayName**.  
Gebruik dit nadat /users/online is uitgeput om een "Members"-sectie weer te geven.  
Cursor-paginering op **commenterName**: de server doorloopt de partiële `{tenantId, urlId, commenterName}` index vanaf **afterName** voortzetten via `$gt`, zonder `$skip`‑kosten.

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| options | GetOfflineUsersOptions | No |  |

## Response

Retourneert: [`Option[PageUsersOfflineResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_offline_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'getOfflineUsers Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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

---