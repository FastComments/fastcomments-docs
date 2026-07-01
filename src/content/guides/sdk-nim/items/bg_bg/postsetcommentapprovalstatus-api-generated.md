## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| options | PostSetCommentApprovalStatusOptions | No |  |

## Отговор

Връща: [`Option[SetCommentApprovedResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_set_comment_approved_response.nim)

## Пример

[inline-code-attrs-start title = 'postSetCommentApprovalStatus пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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

---