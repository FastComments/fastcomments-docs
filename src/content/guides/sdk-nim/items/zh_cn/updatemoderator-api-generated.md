## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|------|------|
| tenantId | string | 是 |  |
| id | string | 否 |  |
| updateModeratorBody | UpdateModeratorBody | 否 |  |

## 响应

返回：[`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 示例

[inline-code-attrs-start title = 'updateModerator 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let body = UpdateModeratorBody(name = "John Doe", email = "john@example.com", isActive = true)
let (apiResult, httpResp) = client.updateModerator(tenantId = "my-tenant-123", id = "mod-456", updateModeratorBody = body)
if apiResult.isSome:
  let result = apiResult.get()
[inline-code-end]