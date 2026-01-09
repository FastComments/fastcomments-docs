## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| afterId | string | Não |  |
| limit | int | Não |  |
| tags | seq[string] | Não |  |

## Resposta

Retorna: [`Option[GetFeedPosts_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_feed_posts200response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getFeedPosts'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getFeedPosts(tenantId = "my-tenant-123", afterId = "post_abc123", limit = 20, tags = @["news", "sports"])
if response.isSome:
  let feed = response.get()
  echo "Feed posts retrieved for tenant my-tenant-123"
else:
  echo "No feed posts returned, HTTP status: ", $httpResponse.status
[inline-code-end]

---