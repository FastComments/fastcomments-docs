---
## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| commentId | string | Tak |  |
| dir | int | Nie |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`Option[GetCommentVoteUserNames_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comment_vote_user_names200response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład getCommentVoteUserNames'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getCommentVoteUserNames(tenantId = "my-tenant-123", commentId = "c_987654321", dir = 0, sso = "")
if response.isSome:
  let res = response.get()
  for userName in res.userNames:
    echo userName
[inline-code-end]

---