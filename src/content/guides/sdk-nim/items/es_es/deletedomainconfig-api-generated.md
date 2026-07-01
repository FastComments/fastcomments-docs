## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| domain | string | No |  |

## Respuesta

Devuelve: [`Option[DeleteDomainConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_domain_config_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'deleteDomainConfig Ejemplo'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.deleteDomainConfig(tenantId = "my-tenant-123", domain = "news.example.com")
if optResp.isSome:
  let resp = optResp.get()
  echo resp
[inline-code-end]