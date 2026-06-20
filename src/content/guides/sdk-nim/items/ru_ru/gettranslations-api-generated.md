## Параметры

| Имя | Тип | Обязательный | Описание |
|------|------|----------|-------------|
| namespace | string | Нет |  |
| component | string | Нет |  |
| locale | string | Нет |  |
| useFullTranslationIds | bool | Нет |  |

## Ответ

Возвращает: [`Option[GetTranslationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_translations_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример getTranslations'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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