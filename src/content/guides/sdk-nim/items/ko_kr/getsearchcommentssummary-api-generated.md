## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| value | string | 아니오 |  |
| filters | string | 아니오 |  |
| searchFilters | string | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`Option[ModerationCommentSearchResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_comment_search_response.nim)

## 예제

[inline-code-attrs-start title = 'getSearchCommentsSummary 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getSearchCommentsSummary(
  value = "news/climate-change-2026",
  filters = "{\"tenantId\":\"my-tenant-123\",\"siteId\":\"main-site\",\"status\":\"approved\"}",
  searchFilters = "author:journalist@news.com OR content:climate",
  sso = "sso-xyz-7890"
)
if response.isSome:
  let summary = response.get()
  echo "Received summary: ", $summary
else:
  echo "No summary returned, HTTP response: ", $httpResponse
[inline-code-end]

---