## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| commentId | string | Tak |  |
| options | GetCommentTextOptions | Nie |  |

## Odpowiedź

Zwraca: [`Option[PublicAPIGetCommentTextResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_public_api_get_comment_text_response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład getCommentText'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResponse, httpResponse) = client.getCommentText(
  tenantId = "my-tenant-123",
  commentId = "comment-456",
  options = GetCommentTextOptions()
)

if maybeResponse.isSome:
  let response = maybeResponse.get()
[inline-code-end]