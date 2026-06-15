מגיבים בעבר בעמוד שאינם מקוונים כרגע. ממוינים לפי displayName.
השתמש בזה לאחר שמיצית את /users/online כדי להציג מדור "חברים".
Cursor pagination על commenterName: השרת מסרק את האינדקס החלקי {tenantId, urlId, commenterName} מהערך afterName והלאה באמצעות $gt — ללא עלות של $skip.

## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| afterName | string | No |  |
| afterUserId | string | No |  |

## תגובה

מחזיר: [`GetOfflineUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOfflineUsers200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getOfflineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_prod_001';
const urlId: string = 'article-2026-06-15-how-ai-impacts';
const afterName: string = 'michael.smith';
const afterUserId: string = 'user_72b9';

const response: GetOfflineUsers200Response = await getOfflineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---