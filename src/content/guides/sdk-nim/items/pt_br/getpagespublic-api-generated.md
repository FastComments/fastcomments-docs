---
Lista de páginas para um tenant. Usado pelo cliente desktop FChat para preencher a sua lista de salas.
Requer que `enableFChat` seja true na configuração customizada resolvida para cada página.
Páginas que requerem SSO são filtradas de acordo com o acesso de grupo do usuário solicitante.

## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| cursor | string | Não |  |
| limit | int | Não |  |
| q | string | Não |  |
| sortBy | PagesSortBy | Não |  |
| hasComments | bool | Não |  |

## Resposta

Retorna: [`Option[GetPublicPagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_public_pages_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getPagesPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPagesPublic(
  tenantId = "my-tenant-123",
  cursor = "",
  limit = 0,
  q = "",
  sortBy = PagesSortBy(0),
  hasComments = false
)

if response.isSome:
  let pages = response.get()
  echo "Retrieved public pages: ", $pages
else:
  echo "No pages returned, HTTP status: ", $httpResponse.status
[inline-code-end]

---