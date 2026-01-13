## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Non |  |

## Réponse

Renvoie : [`Option[GetTenantPackage_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_package200response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de getTenantPackage'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTenantPackage(tenantId = "my-tenant-123", id = "pkg-premium-001")
if response.isSome:
  let pkg = response.get()
  echo pkg
else:
  echo "No package found for tenant"
[inline-code-end]

---