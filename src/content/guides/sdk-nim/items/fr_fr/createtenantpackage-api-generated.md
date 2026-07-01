## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createTenantPackageBody | CreateTenantPackageBody | No |  |

## Réponse

Renvoie : [`Option[CreateTenantPackageResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_tenant_package_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de createTenantPackage'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (responseOpt, httpResponse) = client.createTenantPackage(
  tenantId = "my-tenant-123",
  createTenantPackageBody = CreateTenantPackageBody()
)

if responseOpt.isSome:
  let response = responseOpt.get()
[inline-code-end]