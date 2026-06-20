## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | לא |  |

## תגובה

מחזיר: [`Option[GetUserResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getUser'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUser(tenantId = "my-tenant-123", id = "user-456")
if response.isSome:
  let user = response.get()
  echo user
else:
  echo "User not found"
[inline-code-end]

---