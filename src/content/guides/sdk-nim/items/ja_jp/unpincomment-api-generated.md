## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| commentId | string | はい |  |
| broadcastId | string | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`Option[PinComment_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_pin_comment200response.nim)

## 例

[inline-code-attrs-start title = 'unPinComment の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.unPinComment(tenantId = "my-tenant-123", commentId = "cmt-9f8b7a6", broadcastId = "", sso = "")
if response.isSome:
  let pinResp = response.get()
  echo "Unpinned comment successfully"
else:
  echo "Failed to unpin comment; HTTP response: ", httpResponse
[inline-code-end]

---