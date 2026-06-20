## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| namespace | string | 任意 |  |
| component | string | 任意 |  |
| locale | string | 任意 |  |
| useFullTranslationIds | bool | 任意 |  |

## レスポンス

戻り値: [`Option[GetTranslationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_translations_response.nim)

## 例

[inline-code-attrs-start title = 'getTranslations の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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