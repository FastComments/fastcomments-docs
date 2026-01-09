## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| email | string | 아니오 |  |

## 응답

반환: [`Option[GetSSOUserByEmailAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_sso_user_by_email_api_response.nim)

## 예제

[inline-code-attrs-start title = 'getSSOUserByEmail 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getSSOUserByEmail(tenantId = "my-tenant-123", email = "alice@newsco.com")
if response.isSome:
  let ssoUser = response.get()
  echo "SSO user found: ", ssoUser.email
else:
  echo "No SSO user found. HTTP status: ", httpResponse.status
[inline-code-end]

---