## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | いいえ |  |
| contextUserId | string | いいえ |  |
| isLive | bool | いいえ |  |

## レスポンス

戻り値: [`Option[DeleteCommentResult]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_comment_result.nim)

## 例

[inline-code-attrs-start title = 'deleteComment の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteComment(tenantId = "my-tenant-123", id = "cmt-98765", contextUserId = "user-456", isLive = true)
if response.isSome:
  let result = response.get()
  echo "DeleteCommentResult received"
else:
  echo "No result, HTTP status: ", httpResponse.status
[inline-code-end]

---