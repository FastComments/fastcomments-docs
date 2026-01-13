## パラメーター

| 名前 | Type | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | いいえ |  |

## レスポンス

戻り値: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## 例

[inline-code-attrs-start title = 'deleteNotificationCount の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteNotificationCount(tenantId = "my-tenant-123", id = "notif-456")
if response.isSome:
  let deleted = response.get()
  echo "Deleted notification count:", deleted
else:
  echo "No response body; HTTP response:", httpResponse
[inline-code-end]

---