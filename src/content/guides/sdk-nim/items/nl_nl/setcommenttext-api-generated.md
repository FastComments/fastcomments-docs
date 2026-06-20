## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|--------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| broadcastId | string | Nee |  |
| commentTextUpdateRequest | CommentTextUpdateRequest | Nee |  |
| editKey | string | Nee |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`Option[PublicAPISetCommentTextResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_public_api_set_comment_text_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'setCommentText Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.setCommentText(
  tenantId = "my-tenant-123",
  commentId = "cmt-456789",
  broadcastId = "",
  commentTextUpdateRequest = CommentTextUpdateRequest(text: "Updated comment text to fix a typo and clarify meaning."),
  editKey = "",
  sso = ""
)
if response.isSome:
  let result = response.get()
  discard result
[inline-code-end]

---