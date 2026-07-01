## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|------|------|
| tenantId | string | 예 |  |
| skip | int | 아니오 |  |

## 응답

반환: [`Option[GetSSOUsersResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_sso_users_response.nim)

## 예시

[inline-code-attrs-start title = 'getSSOUsers 예시'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResponse, httpResponse) = client.getSSOUsers(tenantId = "my-tenant-123", skip = 0)
if maybeResponse.isSome:
  let users = maybeResponse.get()
  echo users
else:
  echo "No SSO users found"
echo httpResponse.statusCode
[inline-code-end]