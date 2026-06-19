מגיבים קודמים בדף שאינם מקוונים כרגע. מסודרים לפי displayName.
השתמש בזה לאחר מיצוי /users/online כדי להציג מדור "חברים".
Cursor pagination על commenterName: השרת עובר על האינדקס החלקי {tenantId, urlId, commenterName} החל מ-afterName קדימה באמצעות $gt, ללא עלות של $skip.

## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| urlId | string | כן |  |
| afterName | string | לא |  |
| afterUserId | string | לא |  |

## תגובה

מחזיר: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOfflineResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getOfflineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-9f4b2a6c';
const urlId: string = 'articles/product-launch-2025';

const offlinePageFirst: PageUsersOfflineResponse = await getOfflineUsers(tenantId, urlId);

const afterName: string = 'samantha.r';
const afterUserId: string = 'user_7d3a21f9';
const offlinePageNext: PageUsersOfflineResponse = await getOfflineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---