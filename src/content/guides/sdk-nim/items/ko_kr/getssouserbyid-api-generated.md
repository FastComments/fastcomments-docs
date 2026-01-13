## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 아니요 |  |

## 응답

반환: [`Option[GetSSOUserByIdAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_sso_user_by_id_api_response.nim)

## 예제

[inline-code-attrs-start title = 'getSSOUserById 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getSSOUserById(tenantId = "my-tenant-123", id = "user-789")
if response.isSome:
  let ssoUser: GetSSOUserByIdAPIResponse = response.get()
  echo "SSO user retrieved: ", $ssoUser
else:
  echo "No SSO user found, HTTP status: ", httpResponse.statusCode
[inline-code-end]

---