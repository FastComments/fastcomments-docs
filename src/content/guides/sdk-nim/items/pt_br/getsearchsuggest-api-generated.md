## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenantId | string | Sim |  |
| options | GetSearchSuggestOptions | Não |  |

## Resposta

Retorna: [`Option[ModerationSuggestResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_suggest_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getSearchSuggest'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (suggestOpt, httpResp) = client.getSearchSuggest(
  tenantId = "my-tenant-123",
  options = GetSearchSuggestOptions(),
)

if suggestOpt.isSome:
  let suggest = suggestOpt.get()
  echo suggest
[inline-code-end]