## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| namespace | string | 아니오 |  |
| component | string | 아니오 |  |
| locale | string | 아니오 |  |
| useFullTranslationIds | bool | 아니오 |  |

## 응답

반환: [`Option[GetTranslationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_translations_response.nim)

## 예제

[inline-code-attrs-start title = 'getTranslations 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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