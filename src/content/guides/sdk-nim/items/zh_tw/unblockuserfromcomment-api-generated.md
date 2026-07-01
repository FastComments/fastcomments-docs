## 參數

| 名稱 | 型別 | 必填 | 描述 |
|------|------|------|------|
| tenantId | string | 是 |  |
| id | string | 否 |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | 否 |  |
| options | UnBlockUserFromCommentOptions | 否 |  |

## 回應

回傳: [`Option[UnblockSuccess]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_unblock_success.nim)

## 範例

[inline-code-attrs-start title = 'unBlockUserFromComment 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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