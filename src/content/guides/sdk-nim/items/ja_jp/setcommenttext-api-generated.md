## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| commentId | string | はい |  |
| broadcastId | string | いいえ |  |
| commentTextUpdateRequest | CommentTextUpdateRequest | いいえ |  |
| editKey | string | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`Option[SetCommentText_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_set_comment_text200response.nim)

## 例

[inline-code-attrs-start title = 'setCommentText の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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