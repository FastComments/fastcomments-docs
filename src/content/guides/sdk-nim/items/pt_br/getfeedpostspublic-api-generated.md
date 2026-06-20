req
tenantId
afterId

## Parâmetros

| Name | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenantId | string | Sim |  |
| afterId | string | Não |  |
| limit | int | Não |  |
| tags | seq[string] | Não |  |
| sso | string | Não |  |
| isCrawler | bool | Não |  |
| includeUserInfo | bool | Não |  |

## Resposta

Retorna: [`Option[PublicFeedPostsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_public_feed_posts_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getFeedPostsPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getFeedPostsPublic(
  tenantId = "my-tenant-123",
  afterId = "",
  limit = 0,
  tags = @[],
  sso = "",
  isCrawler = false,
  includeUserInfo = false
)
if response.isSome:
  let feed = response.get()
  discard feed
[inline-code-end]