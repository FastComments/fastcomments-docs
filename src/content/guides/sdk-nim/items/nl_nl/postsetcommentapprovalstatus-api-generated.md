## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|---------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| options | PostSetCommentApprovalStatusOptions | Nee |  |

## Response

Retourneert: [`Option[SetCommentApprovedResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_set_comment_approved_response.nim)

## Example

[inline-code-attrs-start title = 'postSetCommentApprovalStatus Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (approvedOpt, httpResp) = client.postSetCommentApprovalStatus(
  tenantId = "my-tenant-123",
  commentId = "comment-7890",
  options = PostSetCommentApprovalStatusOptions()
)

if approvedOpt.isSome:
  let approved = approvedOpt.get()
  echo approved
[inline-code-end]