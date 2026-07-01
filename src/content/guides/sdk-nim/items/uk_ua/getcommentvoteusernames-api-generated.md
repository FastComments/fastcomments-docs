## Параметри

| Назва | Тип | Обов’язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| commentId | string | Так |  |
| dir | int | Ні |  |
| sso | string = "" | Ні |  |

## Відповідь

Повертає: [`Option[GetCommentVoteUserNamesSuccessResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comment_vote_user_names_success_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад getCommentVoteUserNames'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (voteNamesOpt, httpRes) = client.getCommentVoteUserNames(
  tenantId = "my-tenant-123",
  commentId = "cmt-987654321",
  dir = 0,
  sso = ""
)

if voteNamesOpt.isSome:
  let voteNames = voteNamesOpt.get()
  echo voteNames
[inline-code-end]