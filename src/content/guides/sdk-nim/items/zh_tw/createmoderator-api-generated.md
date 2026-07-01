## 參數

| 名稱 | 型別 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | 是 |  |
| createModeratorBody | CreateModeratorBody | 否 |  |

## 回應

返回: [`Option[CreateModeratorResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_moderator_response.nim)

## 範例

[inline-code-attrs-start title = 'createModerator 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (moderatorRes, httpResp) = client.createModerator(tenantId = "my-tenant-123", createModeratorBody = CreateModeratorBody())
if moderatorRes.isSome:
  let moderator = moderatorRes.get()
[inline-code-end]