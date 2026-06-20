## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| createTenantPackageBody | CreateTenantPackageBody | Non |  |

## Réponse

Renvoie : [`Option[CreateTenantPackageResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_tenant_package_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de createTenantPackage'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createTenantPackage(tenantId = "my-tenant-123", createTenantPackageBody = CreateTenantPackageBody())

if response.isSome:
  let pkg = response.get()
  echo "Created tenant package: ", $pkg
else:
  echo "Failed to create tenant package, HTTP response: ", $httpResponse
[inline-code-end]

---