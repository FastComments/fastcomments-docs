## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| value | string | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`Option[ModerationSiteSearchResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_site_search_response.nim)

## 例

[inline-code-attrs-start title = 'getSearchSites の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getSearchSites(value = "news/2026-olympics", sso = "sso-user-9876-token")
if response.isSome:
  let searchResponse: ModerationSiteSearchResponse = response.get()
  echo "Found sites for search:", searchResponse
[inline-code-end]

---