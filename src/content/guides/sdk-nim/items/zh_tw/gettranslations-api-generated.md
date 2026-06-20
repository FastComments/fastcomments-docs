## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| namespace | string | 否 |  |
| component | string | 否 |  |
| locale | string | 否 |  |
| useFullTranslationIds | bool | 否 |  |

## 回應

回傳: [`Option[GetTranslationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_translations_response.nim)

## 範例

[inline-code-attrs-start title = 'getTranslations Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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