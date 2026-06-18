צופים מקוונים כרגע בעמוד: אנשים שהחיבור שלהם ב-websocket מנוי לעמוד ברגע זה.
מחזיר את anonCount + totalCount (מנויי החדר כולו, כולל צופים אנונימיים שאותם איננו מפרטים).

## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| urlId | string | כן |  |
| afterName | string | לא |  |
| afterUserId | string | לא |  |

## תגובה

מחזיר: [`GetOnlineUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOnlineUsers200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getOnlineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_14f9c3';
const urlId: string = 'article_20250615';
const afterName: string = 'marie.curie';
const afterUserId: string = 'u_92b7';
const result: GetOnlineUsers200Response = await getOnlineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---