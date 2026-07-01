## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Yes |  |
| id | string | No |  |

## Antwort

Rückgabe: [`Option[GetTenantResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_response.nim)

## Beispiel

[inline-code-attrs-start title = 'getTenant Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (tenantResponse, httpResponse) = client.getTenant(tenantId = "my-tenant-123", id = "config-001")
if tenantResponse.isSome:
  let tenant = tenantResponse.get()
  echo tenant
[inline-code-end]