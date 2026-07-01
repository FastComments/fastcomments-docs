## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| domainToUpdate | string | No |  |
| updateDomainConfigParams | UpdateDomainConfigParams | No |  |

## Respuesta

Devuelve: [`Option[PutDomainConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_put_domain_config_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo putDomainConfig'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.putDomainConfig(
  tenantId = "my-tenant-123",
  domainToUpdate = "example.com",
  updateDomainConfigParams = UpdateDomainConfigParams()
)

if optResp.isSome:
  let resp = optResp.get()
  echo resp
[inline-code-end]