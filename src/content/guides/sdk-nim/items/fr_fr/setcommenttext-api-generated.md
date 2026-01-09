## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| commentId | string | Oui |  |
| broadcastId | string | Non |  |
| commentTextUpdateRequest | CommentTextUpdateRequest | Non |  |
| editKey | string | Non |  |
| sso | string | Non |  |

## Réponse

Renvoie : [`Option[SetCommentText_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_set_comment_text200response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de setCommentText'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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

---