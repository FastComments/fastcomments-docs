## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| broadcastId | string | Nee |  |
| commentTextUpdateRequest | CommentTextUpdateRequest | Nee |  |
| editKey | string | Nee |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`Option[SetCommentText_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_set_comment_text200response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'setCommentText Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.setCommentText(
  tenantId = "my-tenant-123",
  commentId = "cmt-7890",
  broadcastId = "broadcast-456",
  commentTextUpdateRequest = CommentTextUpdateRequest(text = "Updated comment text to fix typos and add clarity."),
  editKey = "edit-key-abc123",
  sso = "sso-token-xyz"
)

if response.isSome:
  let updated = response.get()
[inline-code-end]