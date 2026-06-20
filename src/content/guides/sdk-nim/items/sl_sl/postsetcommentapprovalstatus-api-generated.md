## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| commentId | string | Yes |  |
| approved | bool | No |  |
| sso | string | No |  |

## Odgovor

Vrne: [`Option[SetCommentApprovedResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_set_comment_approved_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer postSetCommentApprovalStatus'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postSetCommentApprovalStatus(commentId = "cmt-7890", approved = false, sso = "")
if response.isSome:
  let setResp = response.get()
  discard setResp
[inline-code-end]

---