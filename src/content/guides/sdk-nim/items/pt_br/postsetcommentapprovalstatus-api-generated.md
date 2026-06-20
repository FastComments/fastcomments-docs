## Parâmetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | Sim |  |
| approved | bool | Não |  |
| sso | string | Não |  |

## Resposta

Retorna: [`Option[SetCommentApprovedResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_set_comment_approved_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de postSetCommentApprovalStatus'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postSetCommentApprovalStatus(commentId = "cmt-7890", approved = false, sso = "")
if response.isSome:
  let setResp = response.get()
  discard setResp
[inline-code-end]

---