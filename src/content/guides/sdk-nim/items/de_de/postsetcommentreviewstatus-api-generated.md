## Parameter

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| options | PostSetCommentReviewStatusOptions | No |  |

## Antwort

Rückgabe: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Beispiel

[inline-code-attrs-start title = 'postSetCommentReviewStatus Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiResp, httpResp) = client.postSetCommentReviewStatus(
  tenantId = "my-tenant-123",
  commentId = "cmt-7890",
  options = PostSetCommentReviewStatusOptions()
)

if apiResp.isSome:
  let _ = apiResp.get()
  discard
else:
  discard
[inline-code-end]