## 參數

| 名稱 | 型別 | 必填 | 說明 |
|------|------|----------|-------------|
| value | string | 否 |  |
| filters | string | 否 |  |
| searchFilters | string | 否 |  |
| sso | string | 否 |  |

## 回應

回傳: [`Option[ModerationCommentSearchResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_comment_search_response.nim)

## 範例

[inline-code-attrs-start title = 'getSearchCommentsSummary 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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