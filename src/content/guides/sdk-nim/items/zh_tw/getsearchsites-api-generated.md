## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|-------------|
| value | string | 否 |  |
| sso | string | 否 |  |

## 回應

回傳: [`Option[ModerationSiteSearchResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_site_search_response.nim)

## 範例

[inline-code-attrs-start title = 'getSearchSites 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getSearchSites(value = "news/2026-olympics", sso = "sso-user-9876-token")
if response.isSome:
  let searchResponse: ModerationSiteSearchResponse = response.get()
  echo "Found sites for search:", searchResponse
[inline-code-end]

---