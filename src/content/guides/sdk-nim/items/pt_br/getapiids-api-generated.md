## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| options | GetApiIdsOptions | Não |  |

## Resposta

Retorna: [`Option[ModerationAPIGetCommentIdsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_get_comment_ids_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getApiIds'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let opts = GetApiIdsOptions()
let (maybeResponse, httpResponse) = client.getApiIds(tenantId = "my-tenant-123", options = opts)
if maybeResponse.isSome:
  let response = maybeResponse.get()
  echo response
[inline-code-end]