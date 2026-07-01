## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| commentId | string | Oui |  |
| broadcastId | string | Non |  |
| commentTextUpdateRequest | CommentTextUpdateRequest | Non |  |
| options | SetCommentTextOptions | Non |  |

## Réponse

Retourne : [`Option[PublicAPISetCommentTextResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_public_api_set_comment_text_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple setCommentText'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let commentUpdate = CommentTextUpdateRequest(text: "Updated comment text")
let opts = SetCommentTextOptions()
let (response, httpResponse) = client.setCommentText(
  tenantId = "my-tenant-123",
  commentId = "cmt-456",
  broadcastId = "broadcast-789",
  commentTextUpdateRequest = commentUpdate,
  options = opts)
if response.isSome:
  let result = response.get()
[inline-code-end]