## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| sso | string | לא |  |

## תגובה

מחזיר: [`Option[GetBannedUsersCountResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_banned_users_count_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getCounts'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getCounts(sso = "sso_my-tenant-123_token_AbCdEf123456")
if response.isSome:
  let counts = response.get()
  echo counts
else:
  echo "Request failed with status:", httpResponse.status
[inline-code-end]

---