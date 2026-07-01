## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| options | PostSetCommentApprovalStatusOptions | No |  |

## Svar

Returnerer: [`Option[SetCommentApprovedResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_set_comment_approved_response.nim)

## Eksempel

[inline-code-attrs-start title = 'postSetCommentApprovalStatus Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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