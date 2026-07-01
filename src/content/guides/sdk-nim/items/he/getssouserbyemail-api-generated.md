## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| email | string | לא |  |

## תגובה

מחזיר: [`Option[GetSSOUserByEmailAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_sso_user_by_email_api_response.nim)

## דוגמה

[inline-code-attrs-start title = 'getSSOUserByEmail דוגמה'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.getSSOUserByEmail(tenantId = "my-tenant-123", email = "john.doe@example.com")
if optResp.isSome:
  let user = optResp.get()
  discard user
[inline-code-end]