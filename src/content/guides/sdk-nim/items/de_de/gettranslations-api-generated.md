## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| namespace | string | Nein |  |
| component | string | Nein |  |
| locale | string | Nein |  |
| useFullTranslationIds | bool | Nein |  |

## Antwort

Gibt zurück: [`Option[GetTranslationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_translations_response.nim)

## Beispiel

[inline-code-attrs-start title = 'getTranslations Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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