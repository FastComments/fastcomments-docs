## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| postId | string | Não |  |
| updateFeedPostParams | UpdateFeedPostParams | Não |  |
| options | UpdateFeedPostPublicOptions | Não |  |

## Resposta

Retorna: [`Option[CreateFeedPostResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_feed_post_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo updateFeedPostPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateFeedPostPublic(
  tenantId = "my-tenant-123",
  postId = "post-789",
  updateFeedPostParams = UpdateFeedPostParams(),
  options = UpdateFeedPostPublicOptions()
)

if response.isSome:
  let post = response.get()
[inline-code-end]

---