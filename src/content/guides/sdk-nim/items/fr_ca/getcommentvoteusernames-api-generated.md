## Paramètres

| Name | Type | Requis | Description |
|------|------|--------|-------------|
| tenantId | string | Oui |  |
| commentId | string | Oui |  |
| dir | int | Non |  |
| sso | string | Non |  |

## Réponse

Retourne: [`Option[GetCommentVoteUserNamesSuccessResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comment_vote_user_names_success_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple d\'utilisation de getCommentVoteUserNames'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getCommentVoteUserNames(tenantId = "my-tenant-123", commentId = "cmt-987654", dir = 0, sso = "")
if response.isSome:
  let success: GetCommentVoteUserNamesSuccessResponse = response.get()
  discard success
[inline-code-end]

---