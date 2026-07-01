## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | No |  |

## Respuesta

Devuelve: [`Option[GetTenantPackageResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_package_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'getTenantPackage Ejemplo'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (pkgOpt, httpResp) = client.getTenantPackage(tenantId = "my-tenant-123", id = "premium-plan")
if pkgOpt.isSome:
  let pkg = pkgOpt.get()
  echo pkg
[inline-code-end]