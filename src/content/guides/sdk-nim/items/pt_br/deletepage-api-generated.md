## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| id | string | Não |  |

## Resposta

Retorna: [`Option[DeletePageAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_page_api_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo deletePage'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (deleteRespOpt, httpResp) = client.deletePage(tenantId = "my-tenant-123", id = "news/article-title")
if deleteRespOpt.isSome:
  let deleteResp = deleteRespOpt.get()
  discard deleteResp
discard httpResp
[inline-code-end]