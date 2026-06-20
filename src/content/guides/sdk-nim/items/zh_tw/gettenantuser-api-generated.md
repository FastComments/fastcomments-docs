## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 否 |  |

## 回應

Returns: [`Option[GetTenantUserResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_user_response.nim)

## 範例

[inline-code-attrs-start title = 'getTenantUser 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTenantUser(tenantId = "my-tenant-123", id = "user-789")
if response.isSome:
  let tenantUser = response.get()
  echo "User fetched:", tenantUser
else:
  echo "No user found, HTTP status:", httpResponse.status
[inline-code-end]

---