## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenantId | string | Sim |  |
| page | float64 | Não |  |

## Resposta

Retorna: [`Option[GetHashTagsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_hash_tags_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getHashTags'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getHashTags(tenantId = "news-portal-987", page = 2.0)
if response.isSome:
  let tagsResp = response.get()
  echo "Received hashtags response"
else:
  echo "No hashtags returned"
[inline-code-end]

---