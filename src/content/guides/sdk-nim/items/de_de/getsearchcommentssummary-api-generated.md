## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|---------------|
| tenantId | string | Yes |  |
| options | GetSearchCommentsSummaryOptions | No |  |

## Antwort

Rückgabe: [`Option[ModerationCommentSearchResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_comment_search_response.nim)

## Beispiel

[inline-code-attrs-start title = 'getSearchCommentsSummary Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (summaryOpt, httpResp) = client.getSearchCommentsSummary(
  tenantId = "my-tenant-123",
  options = GetSearchCommentsSummaryOptions()
)

if summaryOpt.isSome:
  let summary = summaryOpt.get()
  echo summary
[inline-code-end]