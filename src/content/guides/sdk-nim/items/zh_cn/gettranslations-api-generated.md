## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| namespace | string | 否 |  |
| component | string | 否 |  |
| locale | string | 否 |  |
| useFullTranslationIds | bool | 否 |  |

## 响应

返回：[`Option[GetTranslationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_translations_response.nim)

## 示例

[inline-code-attrs-start title = 'getTranslations 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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