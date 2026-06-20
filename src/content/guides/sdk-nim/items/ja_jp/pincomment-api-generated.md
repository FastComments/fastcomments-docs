## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| commentId | string | はい |  |
| broadcastId | string | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`Option[ChangeCommentPinStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_change_comment_pin_status_response.nim)

## 例

[inline-code-attrs-start title = 'pinComment の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.pinComment(tenantId = "my-tenant-123", commentId = "cmt-98765", broadcastId = "", sso = "")
if response.isSome:
  let pinnedResp = response.get()
  echo "Pin status updated for comment cmt-98765"
else:
  echo "No response received"
[inline-code-end]

---