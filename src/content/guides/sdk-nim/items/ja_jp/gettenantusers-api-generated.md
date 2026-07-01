## パラメータ

| Name | Type | Required | 説明 |
|------|------|----------|------|
| tenantId | string | Yes |  |
| skip | float64 | No |  |

## レスポンス

返り値: [`Option[GetTenantUsersResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_users_response.nim)

## 例

[inline-code-attrs-start title = 'getTenantUsers の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTenantUsers(tenantId = "my-tenant-123", skip = 0.0)
if response.isSome:
  let data = response.get()
  echo data
[inline-code-end]