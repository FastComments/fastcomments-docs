req
tenantId
urlId

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| urlId | string | Sim |  |
| options | GetCommentsPublicOptions | Não |  |

## Resposta

Retorna: [`Option[GetCommentsResponseWithPresencePublicComment]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_response_with_presence_public_comment.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getCommentsPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.getCommentsPublic(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  options = GetCommentsPublicOptions(
    page = 0,
    pageSize = 10,
    includeDeleted = false,
    tags = @[]
  )
)

if optResp.isSome:
  let resp = optResp.get()
  echo resp
[inline-code-end]

---