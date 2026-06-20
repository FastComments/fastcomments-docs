## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| createTenantPackageBody | CreateTenantPackageBody | No |  |

## Risposta

Restituisce: [`Option[CreateTenantPackageResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_tenant_package_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio di createTenantPackage'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.createTenantPackage(tenantId = "my-tenant-123", createTenantPackageBody = CreateTenantPackageBody())

if response.isSome:
  let pkg = response.get()
  echo "Created tenant package: ", $pkg
else:
  echo "Failed to create tenant package, HTTP response: ", $httpResponse
[inline-code-end]

---