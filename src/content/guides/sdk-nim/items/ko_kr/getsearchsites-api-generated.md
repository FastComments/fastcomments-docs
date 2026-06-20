## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|------|------|
| value | string | 아니요 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`Option[ModerationSiteSearchResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_site_search_response.nim)

## 예제

[inline-code-attrs-start title = 'getSearchSites 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getSearchSites(value = "news/2026-olympics", sso = "sso-user-9876-token")
if response.isSome:
  let searchResponse: ModerationSiteSearchResponse = response.get()
  echo "Found sites for search:", searchResponse
[inline-code-end]

---