## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|---------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| dir | int | Nein |  |
| sso | string = "" | Nein |  |

## Antwort

Rückgabe: [`Option[GetCommentVoteUserNamesSuccessResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comment_vote_user_names_success_response.nim)

## Beispiel

[inline-code-attrs-start title = 'getCommentVoteUserNames Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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