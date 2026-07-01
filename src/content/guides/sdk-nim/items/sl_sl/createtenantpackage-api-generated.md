## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| createTenantPackageBody | CreateTenantPackageBody | Ne |  |

## Odgovor

Vrne: [`Option[CreateTenantPackageResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_tenant_package_response.nim)

## Primer

[inline-code-attrs-start title = 'createTenantPackage Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (responseOpt, httpResponse) = client.createTenantPackage(
  tenantId = "my-tenant-123",
  createTenantPackageBody = CreateTenantPackageBody()
)

if responseOpt.isSome:
  let response = responseOpt.get()
[inline-code-end]