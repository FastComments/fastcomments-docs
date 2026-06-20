מידע משתמשים בכמות גדולה עבור tenant. בהתחשב ב-userIds, מחזיר מידע להצגה מ-User / SSOUser.
משמש את הווידג'ט של התגובות להעשיר משתמשים שזה עתה הופיעו באמצעות אירוע נוכחות.
אין הקשר של דף: פרטיות נאכפת באופן אחיד (פרופילים פרטיים מוסתרים).

## פרמטרים

| שם | סוג | דרוש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| ids | string | לא |  |

## תגובה

מחזיר: [`Option[PageUsersInfoResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_info_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getUsersInfo'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUsersInfo(tenantId = "my-tenant-123", ids = "user-42,user-87")
if response.isSome:
  let usersInfo = response.get()
  echo "Retrieved users info:", usersInfo
else:
  echo "No users info returned. HTTP status:", httpResponse.status
[inline-code-end]

---