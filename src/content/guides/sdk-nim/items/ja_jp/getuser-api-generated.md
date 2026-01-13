## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | いいえ |  |

## レスポンス

戻り値: [`Option[GetUser_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user200response.nim)

## 例

[inline-code-attrs-start title = 'getUser の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUser(tenantId = "my-tenant-123", id = "user-9876")
if response.isSome:
  let user = response.get()
  echo "User:", user
else:
  echo "No user found. HTTP response:", httpResponse
[inline-code-end]

---