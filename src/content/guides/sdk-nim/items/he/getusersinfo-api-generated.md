מידע משתמשים בכמות עבור טננט. בהינתן userIds, מחזיר מידע תצוגה מ-User / SSOUser.
משמש את ווידג'ט התגובות להעשרת משתמשים שהופיעו זה עתה באמצעות אירוע נוכחות.
ללא הקשר עמוד: הפרטיות נאכפת באופן אחיד (פרופילים פרטיים מוסתרים).

## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| ids | string | No |  |

## תגובה

מחזיר: [`Option[PageUsersInfoResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_info_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getUsersInfo'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (usersInfoOpt, httpResp) = client.getUsersInfo(tenantId = "my-tenant-123", ids = "user42")
if usersInfoOpt.isSome:
  let usersInfo = usersInfoOpt.get()
[inline-code-end]