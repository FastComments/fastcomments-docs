## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| domain | string | No |  |

## Respuesta

Devuelve: [`Option[GetDomainConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_domain_config_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getDomainConfig'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getDomainConfig(tenantId = "my-tenant-123", domain = "news/top-story-2026")
if response.isSome:
  let cfg = response.get()
  discard cfg
else:
  discard httpResponse
[inline-code-end]

---