## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| namespace | string | Hayır |  |
| component | string | Hayır |  |
| locale | string | Hayır |  |
| useFullTranslationIds | bool | Hayır |  |

## Yanıt

Döndürür: [`Option[GetTranslationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_translations_response.nim)

## Örnek

[inline-code-attrs-start title = 'getTranslations Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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