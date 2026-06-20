## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| namespace | string | Не |  |
| component | string | Не |  |
| locale | string | Не |  |
| useFullTranslationIds | bool | Не |  |

## Одговор

Враћа: [`Option[GetTranslationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_translations_response.nim)

## Пример

[inline-code-attrs-start title = 'getTranslations Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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