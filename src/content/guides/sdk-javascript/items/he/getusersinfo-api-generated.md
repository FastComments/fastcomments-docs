---
מידע קבוצתי של משתמשים עבור שוכר. בהתבסס על userIds, מחזיר מידע להצגה מ־User / SSOUser.
נמצא בשימוש על ידי ווידג'ט התגובות כדי להעשיר משתמשים שהופיעו זה עתה באמצעות אירוע נוכחות.
אין הקשר של דף: פרטיות נאכפת באופן אחיד (פרופילים פרטיים מוסתרים).

## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| ids | string | Yes |  |

## תגובה

מחזיר: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersInfoResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getUsersInfo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_78f9';
const ids: string = 'user_10234,user_10235,user_10236';
const usersInfo: PageUsersInfoResponse = await getUsersInfo(tenantId, ids);
// getUsersInfo דורשת רק את tenantId ו-ids; פרמטרים אופציונליים אינם רלוונטיים כאן.
[inline-code-end]

---