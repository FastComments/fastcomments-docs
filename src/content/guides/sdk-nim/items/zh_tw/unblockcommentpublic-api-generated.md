## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | 否 |  |
| sso | string | 否 |  |

## 回應

回傳: [`Option[UnblockSuccess]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_unblock_success.nim)

## 範例

[inline-code-attrs-start title = 'unBlockCommentPublic 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.unBlockCommentPublic(tenantId = "my-tenant-123", commentId = "cmt-987654321", publicBlockFromCommentParams = PublicBlockFromCommentParams(), sso = "")
if response.isSome:
  let unblockResult = response.get()
  discard unblockResult
else:
  discard httpResponse
[inline-code-end]

---