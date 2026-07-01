---
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|------|------|
| tenantId | string | はい |  |
| id | string | いいえ |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | いいえ |  |
| options | UnBlockUserFromCommentOptions | いいえ |  |

## レスポンス

戻り値: [`Option[UnblockSuccess]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_unblock_success.nim)

## 例

[inline-code-attrs-start title = 'unBlockUserFromComment の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.unBlockUserFromComment(
  tenantId = "my-tenant-123",
  id = "comment-456",
  unBlockFromCommentParams = UnBlockFromCommentParams(userId = "user-789", commentId = "cmt-321"),
  options = UnBlockUserFromCommentOptions(),
)

if response.isSome:
  let unblockSuccess = response.get()
[inline-code-end]

---