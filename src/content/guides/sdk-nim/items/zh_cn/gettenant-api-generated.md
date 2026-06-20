## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| id | string | 否 |  |

## 响应

返回：[`Option[GetTenantResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_response.nim)

## 示例

[inline-code-attrs-start title = 'getTenant 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTenant(tenantId = "my-tenant-123", id = "tenant-789")
if response.isSome:
  let tenant = response.get()
  discard tenant
else:
  discard httpResponse
[inline-code-end]

---