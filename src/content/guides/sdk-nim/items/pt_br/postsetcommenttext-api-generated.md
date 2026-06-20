## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| commentId | string | Sim |  |
| setCommentTextParams | SetCommentTextParams | Não |  |
| sso | string | Não |  |

## Resposta

Retorna: [`Option[SetCommentTextResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_set_comment_text_response.nim)

## Exemplo

[inline-code-attrs-start title = 'postSetCommentText Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postSetCommentText(commentId = "comment-4821",
  setCommentTextParams = SetCommentTextParams(text = "Updated comment to clarify the main point and fix a typo."),
  sso = "sso-user-8f3b9c")

if response.isSome:
  let setCommentResp = response.get()
  echo "Received SetCommentTextResponse"
[inline-code-end]

---