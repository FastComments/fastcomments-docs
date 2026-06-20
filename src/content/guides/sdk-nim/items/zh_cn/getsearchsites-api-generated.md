---
## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| value | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回：[`Option[ModerationSiteSearchResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_site_search_response.nim)

## 示例

[inline-code-attrs-start title = 'getSearchSites 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getSearchSites(value = "news/2026-olympics", sso = "sso-user-9876-token")
if response.isSome:
  let searchResponse: ModerationSiteSearchResponse = response.get()
  echo "Found sites for search:", searchResponse
[inline-code-end]

---