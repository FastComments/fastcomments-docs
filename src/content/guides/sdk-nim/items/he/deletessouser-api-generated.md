## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | לא |  |
| deleteComments | bool | לא |  |
| commentDeleteMode | string | לא |  |

## תגובה

מחזיר: [`Option[DeleteSSOUserAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_sso_user_api_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-deleteSSOUser'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteSSOUser(tenantId = "my-tenant-123", id = "sso-user-9876", deleteComments = true, commentDeleteMode = "hard")
if response.isSome:
  let deleted = response.get()
  discard deleted
else:
  discard httpResponse
[inline-code-end]

---