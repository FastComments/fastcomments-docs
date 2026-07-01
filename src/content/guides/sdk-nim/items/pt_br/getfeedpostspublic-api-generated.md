req  
tenantId  
afterId  

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenantId | string | Sim |  |
| options | GetFeedPostsPublicOptions | Não |  |

## Resposta

Retorna: [`Option[PublicFeedPostsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_public_feed_posts_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getFeedPostsPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]  
[inline-code-start]  
let (feedResponseOpt, httpResponse) = client.getFeedPostsPublic(tenantId = "my-tenant-123", options = GetFeedPostsPublicOptions())  
if feedResponseOpt.isSome:  
  let feedResponse = feedResponseOpt.get()  
[inline-code-end]