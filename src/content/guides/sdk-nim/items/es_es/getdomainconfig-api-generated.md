## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| domain | string | No |  |

## Respuesta

Devuelve: [`Option[GetDomainConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_domain_config_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo getDomainConfig'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (configOpt, httpResp) = client.getDomainConfig(tenantId = "my-tenant-123", domain = "news.example.com")
if configOpt.isSome:
  let cfg = configOpt.get()
  discard cfg
[inline-code-end]