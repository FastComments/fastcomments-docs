## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| id | string | No |  |

## Risposta

Restituisce: [`Option[GetTenantPackageResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_package_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio getTenantPackage'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (pkgOpt, httpResp) = client.getTenantPackage(tenantId = "my-tenant-123", id = "premium-plan")
if pkgOpt.isSome:
  let pkg = pkgOpt.get()
  echo pkg
[inline-code-end]

---