## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| commentId | string | はい |  |
| broadcastId | string | いいえ |  |
| editKey | string | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`Option[DeleteCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_comment_public200response.nim)

## 例

[inline-code-attrs-start title = 'deleteCommentPublic の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteCommentPublic(
  tenantId = "my-tenant-123",
  commentId = "cmt-987654",
  broadcastId = "",
  editKey = "",
  sso = ""
)
if response.isSome:
  let deleted = response.get()
  echo "Delete succeeded"
  echo "HTTP status: ", httpResponse.status
else:
  echo "Delete failed, HTTP status: ", httpResponse.status
[inline-code-end]

---