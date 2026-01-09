## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| commentId | string | Tak |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Nie |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`Option[UnBlockCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_un_block_comment_public200response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład unBlockCommentPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.unBlockCommentPublic(
  tenantId = "news-site-456",
  commentId = "comment-abc123",
  publicBlockFromCommentParams = PublicBlockFromCommentParams{},
  sso = ""
)
if response.isSome:
  let unblocked = response.get()
  discard unblocked
else:
  discard httpResponse
[inline-code-end]

---