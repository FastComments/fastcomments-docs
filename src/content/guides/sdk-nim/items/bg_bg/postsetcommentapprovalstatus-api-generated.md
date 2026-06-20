## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| commentId | string | Да |  |
| approved | bool | Не |  |
| sso | string | Не |  |

## Отговор

Връща: [`Option[SetCommentApprovedResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_set_comment_approved_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример за postSetCommentApprovalStatus'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postSetCommentApprovalStatus(commentId = "cmt-7890", approved = false, sso = "")
if response.isSome:
  let setResp = response.get()
  discard setResp
[inline-code-end]

---