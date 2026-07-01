## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | いいえ |  |

## レスポンス

戻り値: [`Option[GetUserResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_response.nim)

## 例

[inline-code-attrs-start title = 'getUser の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (userOpt, httpResp) = client.getUser(tenantId = "my-tenant-123", id = "user-456")
if userOpt.isSome:
  let user = userOpt.get()
  echo user
else:
  echo "User not found"
[inline-code-end]

---