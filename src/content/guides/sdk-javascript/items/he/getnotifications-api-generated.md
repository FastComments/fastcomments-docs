## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| userId | string | לא |  |
| urlId | string | לא |  |
| fromCommentId | string | לא |  |
| viewed | boolean | לא |  |
| type | string | לא |  |
| skip | number | לא |  |

## תגובה

מחזיר: [`GetNotifications200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotifications200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getNotifications'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_84b3f2";
const userId: string = "user_1279";
const urlId: string = "https://www.example.com/articles/2026/03/25/new-feature";
const fromCommentId: string = "cmt_5421";
const viewed: boolean = false;
const type: string = "mention";
const skip: number = 0;
const notifications: GetNotifications200Response = await getNotifications(tenantId, userId, urlId, fromCommentId, viewed, type, skip);
[inline-code-end]

---