## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| email | string | No |  |

## 응답

Returns: [`Option[GetSSOUserByEmailAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_sso_user_by_email_api_response.nim)

## 예시

[inline-code-attrs-start title = 'getSSOUserByEmail 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.getSSOUserByEmail(tenantId = "my-tenant-123", email = "john.doe@example.com")
if optResp.isSome:
  let user = optResp.get()
  discard user
[inline-code-end]