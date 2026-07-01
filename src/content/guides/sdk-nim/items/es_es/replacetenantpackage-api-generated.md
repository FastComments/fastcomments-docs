## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | No |  |
| replaceTenantPackageBody | ReplaceTenantPackageBody | No |  |

## Respuesta

Devuelve: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo replaceTenantPackage'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.replaceTenantPackage(
  tenantId = "my-tenant-123",
  id = "pkg-456",
  replaceTenantPackageBody = ReplaceTenantPackageBody()
)
if optResp.isSome:
  let resp = optResp.get()
  discard resp
[inline-code-end]