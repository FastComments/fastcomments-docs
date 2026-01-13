## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 否 |  |

## 响应

返回: [`Option[GetModerator_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_moderator200response.nim)

## 示例

[inline-code-attrs-start title = 'getModerator 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getModerator(tenantId = "my-tenant-123", id = "mod-98765")
if response.isSome:
  let moderator = response.get()
  discard moderator
[inline-code-end]

---