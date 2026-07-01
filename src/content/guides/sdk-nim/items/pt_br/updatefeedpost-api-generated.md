## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenantId | string | Sim |  |
| id | string | Não |  |
| feedPost | FeedPost | Não |  |

## Resposta

Retorna: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de updateFeedPost'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let feedPost = FeedPost(
  title: "Breaking News",
  content: "Updated story content",
  tags: @["news", "update"]
)

let (apiRes, httpRes) = client.updateFeedPost(
  tenantId = "my-tenant-123",
  id = "post-456",
  feedPost = feedPost
)

if apiRes.isSome:
  let res = apiRes.get()
[inline-code-end]