## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| namespace | string | Ні |  |
| component | string | Ні |  |
| locale | string | Ні |  |
| useFullTranslationIds | bool | Ні |  |

## Відповідь

Повертає: [`Option[GetTranslationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_translations_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад використання getTranslations'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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