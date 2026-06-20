## Parâmetros

| Name | Type | Obrigatório | Descrição |
|------|------|----------|-------------|
| namespace | string | Não |  |
| component | string | Não |  |
| locale | string | Não |  |
| useFullTranslationIds | bool | Não |  |

## Resposta

Retorna: [`Option[GetTranslationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_translations_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getTranslations'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTranslations(
  namespace = "news-site",
  component = "article-comments",
  locale = "en-US",
  useFullTranslationIds = false
)
if response.isSome:
  let translations = response.get()
  discard translations
else:
  echo "No translations available"
[inline-code-end]

---