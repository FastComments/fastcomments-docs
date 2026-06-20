## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| postIds | seq[string] | Não |  |
| sso | string | Não |  |

## Resposta

Retorna: [`Option[FeedPostsStatsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_feed_posts_stats_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getFeedPostsStats'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getFeedPostsStats(tenantId = "my-tenant-123", postIds = @["news/article-2026", "opinion/market-trends"], sso = "")
if response.isSome:
  let stats = response.get()
  echo "Received feed posts stats for tenant:", " my-tenant-123"
  discard stats
[inline-code-end]

---