## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| namespace | string | Nie |  |
| component | string | Nie |  |
| locale | string | Nie |  |
| useFullTranslationIds | bool | Nie |  |

## Odpowiedź

Zwraca: [`Option[GetTranslationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_translations_response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład getTranslations'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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