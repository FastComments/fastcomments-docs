## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 否 |  |
| redirectURL | string = "" | 否 |  |

## 响应

返回：[`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 示例

[inline-code-attrs-start title = 'sendLoginLink 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResp, httpResp) = client.sendLoginLink(
  tenantId = "my-tenant-123",
  id = "user-456",
  redirectURL = "https://myapp.example.com/login-success"
)

if maybeResp.isSome:
  let emptyResp = maybeResp.get()
[inline-code-end]