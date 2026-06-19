צופים מקוונים כרגע בדף: אנשים שסשן websocket שלהם מנוי על הדף ברגע זה.
מחזיר את anonCount + totalCount (מנויים בחדר כולו, כולל צופים אנונימיים שאותם איננו מונים).

## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| urlId | string | כן |  |
| afterName | string | לא |  |
| afterUserId | string | לא |  |

## תגובה

מחזיר: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOnlineResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getOnlineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f3c2b7';
const urlId: string = 'article-2026-06-19-site-update';
const afterName: string = 'michael.hansen';
const afterUserId: string = 'user_00421';
const onlineUsers: PageUsersOnlineResponse = await getOnlineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---