---
## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 否 |  |
| sure | string | 否 |  |

## 响应

返回：[`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## 示例

[inline-code-attrs-start title = 'deleteTenant 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteTenant(tenantId = "my-tenant-123", id = "", sure = "")

if response.isSome:
  let flagResp = response.get()
  echo "Tenant deletion response received for tenant: ", "my-tenant-123"
  discard flagResp
else:
  echo "No response body returned for tenant deletion"
[inline-code-end]

---