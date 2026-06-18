מידע קבוצתי על משתמשים עבור שוכר. בהינתן userIds, מחזיר מידע תצוגה מ-User / SSOUser.
משמש את ווידג'ט התגובות להעשיר משתמשים שהופיעו זה עתה באמצעות אירוע נוכחות.
אין הקשר של דף: הפרטיות נאכפת באופן אחיד (פרופילים פרטיים מוסתרים).

## פרמטרים

| שם | סוג | דרוש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| ids | string | כן |  |

## תגובה

מחזיר: [`GetUsersInfo200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUsersInfo200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getUsersInfo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-007';
const userIdsList: string[] = ['user_12a', 'user_34b', 'user_56c'];
const separator: string | undefined = undefined; // אופציונלי; אם undefined אז כברירת מחדל פסיק
const ids: string = userIdsList.join(separator ?? ',');
const usersInfo: GetUsersInfo200Response = await getUsersInfo(tenantId, ids);
[inline-code-end]

---