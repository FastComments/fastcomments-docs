## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 아니요 |  |

## 응답

반환: [`Option[GetUserResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_response.nim)

## 예제

[inline-code-attrs-start title = 'getUser 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUser(tenantId = "my-tenant-123", id = "user-456")
if response.isSome:
  let user = response.get()
  echo user
else:
  echo "User not found"
[inline-code-end]

---