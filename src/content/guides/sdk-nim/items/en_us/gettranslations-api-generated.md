## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| namespace | string | No |  |
| component | string | No |  |
| locale | string | No |  |
| useFullTranslationIds | bool | No |  |

## Response

Returns: [`Option[GetTranslationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_translations_response.nim)

## Example

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
