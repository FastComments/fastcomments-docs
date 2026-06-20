## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| commentId | string | Da |  |
| approved | bool | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`Option[SetCommentApprovedResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_set_comment_approved_response.nim)

## Primjer

[inline-code-attrs-start title = 'Primjer postSetCommentApprovalStatus'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postSetCommentApprovalStatus(commentId = "cmt-7890", approved = false, sso = "")
if response.isSome:
  let setResp = response.get()
  discard setResp
[inline-code-end]

---