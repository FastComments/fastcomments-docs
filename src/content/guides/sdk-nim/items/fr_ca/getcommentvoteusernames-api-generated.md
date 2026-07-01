## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| dir | int | No |  |
| sso | string = "" | No |  |

## Réponse

Retourne : [`Option[GetCommentVoteUserNamesSuccessResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comment_vote_user_names_success_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple getCommentVoteUserNames'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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