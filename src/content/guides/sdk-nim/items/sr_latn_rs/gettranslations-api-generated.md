## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| namespace | string | Ne |  |
| component | string | Ne |  |
| locale | string | Ne |  |
| useFullTranslationIds | bool | Ne |  |

## Odgovor

Vraća: [`Option[GetTranslationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_translations_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer getTranslations'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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