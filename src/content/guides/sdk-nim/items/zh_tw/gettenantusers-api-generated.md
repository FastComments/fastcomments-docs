## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| skip | float64 | 否 |  |

## 回應

回傳: [`Option[GetTenantUsers_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_users200response.nim)

## 範例

[inline-code-attrs-start title = 'getTenantUsers 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTenantUsers(tenantId = "my-tenant-123", skip = 0.0)
if response.isSome:
  let tenantUsers = response.get()
  echo "Fetched tenant users for my-tenant-123"
  discard tenantUsers
else:
  echo "No users returned"
  discard httpResponse
[inline-code-end]

---