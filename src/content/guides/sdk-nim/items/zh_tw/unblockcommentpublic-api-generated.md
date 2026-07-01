## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | 否 |  |
| sso | string = "" | 否 |  |

## 回應

返回：[`Option[UnblockSuccess]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_unblock_success.nim)

## 範例

[inline-code-attrs-start title = 'unBlockCommentPublic 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (unblockResult, httpResp) = client.unBlockCommentPublic(
  tenantId = "my-tenant-123",
  commentId = "comment-7890",
  publicBlockFromCommentParams = PublicBlockFromCommentParams(),
  sso = ""
)

if unblockResult.isSome:
  let result = unblockResult.get()
[inline-code-end]

---