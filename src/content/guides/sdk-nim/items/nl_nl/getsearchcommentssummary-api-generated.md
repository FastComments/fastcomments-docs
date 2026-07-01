## Parameters

| Name | Type | Required | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | GetSearchCommentsSummaryOptions | No |  |

## Response

Retourneert: [`Option[ModerationCommentSearchResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_comment_search_response.nim)

## Example

[inline-code-attrs-start title = 'getSearchCommentsSummary Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (summaryOpt, httpResp) = client.getSearchCommentsSummary(
  tenantId = "my-tenant-123",
  options = GetSearchCommentsSummaryOptions()
)

if summaryOpt.isSome:
  let summary = summaryOpt.get()
  echo summary
[inline-code-end]