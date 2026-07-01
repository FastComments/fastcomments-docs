## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| banUserUndoParams | BanUserUndoParams | No |  |
| sso | string = "" | No |  |

## 响应

Returns: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 示例

[inline-code-attrs-start title = 'postBanUserUndo 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiResp, httpResp) = client.postBanUserUndo(
  tenantId = "my-tenant-123",
  banUserUndoParams = BanUserUndoParams(userId = "user-456"),
  sso = ""
)

if apiResp.isSome:
  let _ = apiResp.get()
[inline-code-end]