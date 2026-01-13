## Parameter

| Name | Type | Erforderlich | Beschreibung |
|------|------|--------------|-------------|
| tenantId | string | Ja |  |
| skip | float64 | Nein |  |

## Antwort

Gibt zurück: [`Option[GetTenantUsers_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_users200response.nim)

## Beispiel

[inline-code-attrs-start title = 'getTenantUsers Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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