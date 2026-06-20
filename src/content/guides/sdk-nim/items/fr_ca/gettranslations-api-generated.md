## Paramètres

| Name | Type | Requis | Description |
|------|------|--------|-------------|
| namespace | string | Non |  |
| component | string | Non |  |
| locale | string | Non |  |
| useFullTranslationIds | bool | Non |  |

## Réponse

Renvoie: [`Option[GetTranslationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_translations_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de getTranslations'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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