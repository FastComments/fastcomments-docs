## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|--------|------|-----------|--------------|
| tenantId | string | Sí |  |
| addDomainConfigParams | AddDomainConfigParams | No |  |

## Respuesta

Devuelve: [`Option[AddDomainConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_add_domain_config_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo addDomainConfig'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (respOpt, httpResp) = client.addDomainConfig(
  tenantId = "my-tenant-123",
  addDomainConfigParams = default(AddDomainConfigParams)
)

if respOpt.isSome:
  let cfg = respOpt.get()
[inline-code-end]