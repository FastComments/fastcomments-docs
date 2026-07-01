## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| namespace | string | 否 |  |
| component | string | 否 |  |
| options | GetTranslationsOptions | 否 |  |

## 回傳

Returns: [`Option[GetTranslationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_translations_response.nim)

## 範例

[inline-code-attrs-start title = 'getTranslations 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let opts = GetTranslationsOptions()
let (maybeResp, httpResp) = client.getTranslations(namespace = "my-tenant-123", component = "news/article-title", options = opts)
if maybeResp.isSome:
  let resp = maybeResp.get()
  echo resp
[inline-code-end]