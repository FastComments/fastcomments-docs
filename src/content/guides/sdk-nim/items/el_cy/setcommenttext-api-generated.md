## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| commentId | string | Ναι |  |
| broadcastId | string | Όχι |  |
| commentTextUpdateRequest | CommentTextUpdateRequest | Όχι |  |
| editKey | string | Όχι |  |
| sso | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`Option[SetCommentText_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_set_comment_text200response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα setCommentText'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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