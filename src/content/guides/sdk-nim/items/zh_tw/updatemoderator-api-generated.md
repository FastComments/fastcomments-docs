## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| updateModeratorBody | UpdateModeratorBody | No |  |

## 回應

返回: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 範例

[inline-code-attrs-start title = 'updateModerator 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let body = UpdateModeratorBody(name = "John Doe", email = "john@example.com", isActive = true)
let (apiResult, httpResp) = client.updateModerator(tenantId = "my-tenant-123", id = "mod-456", updateModeratorBody = body)
if apiResult.isSome:
  let result = apiResult.get()
[inline-code-end]