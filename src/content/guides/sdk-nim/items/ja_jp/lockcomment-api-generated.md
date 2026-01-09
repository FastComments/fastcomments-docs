## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| commentId | string | はい |  |
| broadcastId | string | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

返却: [`Option[LockComment_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_lock_comment200response.nim)

## 例

[inline-code-attrs-start title = 'lockComment の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.lockComment(
  tenantId = "my-tenant-123",
  commentId = "cmt-98765",
  broadcastId = "",
  sso = ""
)
if response.isSome:
  let lockResp = response.get()
  discard lockResp
[inline-code-end]

---