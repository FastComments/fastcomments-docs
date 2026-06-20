## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| commentId | string | はい |  |
| broadcastId | string | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 例

[inline-code-attrs-start title = 'unLockComment の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let tenantId = "my-tenant-123"
let commentId = "cmt-987654321"
let (response, httpResponse) = client.unLockComment(
  tenantId = tenantId,
  commentId = commentId,
  broadcastId = "",
  sso = ""
)
if response.isSome:
  let apiResp = response.get()
  echo "Unlocked comment ", commentId, " for tenant ", tenantId
else:
  echo "Unlock failed, HTTP status: ", $httpResponse.status
[inline-code-end]

---