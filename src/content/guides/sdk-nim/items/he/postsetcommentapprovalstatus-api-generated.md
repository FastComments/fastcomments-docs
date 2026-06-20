## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | כן |  |
| approved | bool | לא |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`Option[SetCommentApprovedResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_set_comment_approved_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה עבור postSetCommentApprovalStatus'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postSetCommentApprovalStatus(commentId = "cmt-7890", approved = false, sso = "")
if response.isSome:
  let setResp = response.get()
  discard setResp
[inline-code-end]

---