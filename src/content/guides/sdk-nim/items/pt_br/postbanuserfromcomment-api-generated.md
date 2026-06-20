## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| commentId | string | Sim |  |
| banEmail | bool | Não |  |
| banEmailDomain | bool | Não |  |
| banIP | bool | Não |  |
| deleteAllUsersComments | bool | Não |  |
| bannedUntil | string | Não |  |
| isShadowBan | bool | Não |  |
| updateId | string | Não |  |
| banReason | string | Não |  |
| sso | string | Não |  |

## Resposta

Retorna: [`Option[BanUserFromCommentResult]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_ban_user_from_comment_result.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de postBanUserFromComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postBanUserFromComment(
  commentId = "cmt-8f3a1b",
  banEmail = false,
  banEmailDomain = false,
  banIP = false,
  deleteAllUsersComments = false,
  bannedUntil = "",
  isShadowBan = false,
  updateId = "",
  banReason = "",
  sso = ""
)
if response.isSome:
  let banResult = response.get()
  discard banResult
else:
  echo "No ban result returned"
[inline-code-end]

---