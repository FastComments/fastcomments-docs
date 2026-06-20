---
## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| addDomainConfigParams | AddDomainConfigParams | No |  |

## Respuesta

Devuelve: [`Option[AddDomainConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_add_domain_config_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de addDomainConfig'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let config = AddDomainConfigParams(
  domain: "comments.example-news.com",
  enabled: true,
  allowedOrigins: @["https://www.example-news.com", "https://m.example-news.com"],
  commentsPath: "/news/world/election-coverage",
  priority: 5
)
let (response, httpResponse) = client.addDomainConfig(tenantId = "my-tenant-123", addDomainConfigParams = config)
if response.isSome:
  let created = response.get()
  echo "Created domain config:", created
else:
  echo "Failed to create domain config, HTTP status:", httpResponse.status.code
[inline-code-end]

---