## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| commentId | string | Tak |  |
| voteId | string | Nie |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`Option[VoteDeleteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_delete_response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład deleteModerationVote'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteModerationVote(commentId = "my-tenant-123/news/article-title/comment-987", voteId = "vote-456", sso = "sso-token-abc")
if response.isSome:
  let voteResp = response.get()
  echo "Vote deleted:", voteResp
else:
  echo "Delete failed:", httpResponse
[inline-code-end]

---