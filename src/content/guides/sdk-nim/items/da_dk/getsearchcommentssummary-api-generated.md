## Parametre

| Name | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| value | string | Nej |  |
| filters | string | Nej |  |
| searchFilters | string | Nej |  |
| sso | string | Nej |  |

## Svar

Returnerer: [`Option[ModerationCommentSearchResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_comment_search_response.nim)

## Eksempel

[inline-code-attrs-start title = 'getSearchCommentsSummary Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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