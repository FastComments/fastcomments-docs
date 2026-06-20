## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| skip | float64 | Ne |  |

## Odgovor

Vrača: [`Option[GetTenantUsersResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_users_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer getTenantUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTenantUsers(tenantId = "my-tenant-123", skip = 0.0)

if response.isSome:
  let tenantUsers = response.get()
  echo "Retrieved tenant users"
else:
  echo "No tenant users returned"
[inline-code-end]

---