## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| userId | string | לא |  |
| urlId | string | לא |  |
| fromCommentId | string | לא |  |
| viewed | boolean | לא |  |
| type | string | לא |  |

## תגובה

מחזיר: [`GetNotificationCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetNotificationCountResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getNotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f3b2a';
const userId: string = 'user_7421';
const urlId: string = 'https://news.example.com/articles/2026/06/19/ai-update';
const fromCommentId: string = 'cmt_5a1d2f';
const viewed: boolean = false;
const type: string = 'mention';

const notificationCount: GetNotificationCountResponse = await getNotificationCount(
  tenantId,
  userId,
  urlId,
  fromCommentId,
  viewed,
  type
);
[inline-code-end]

---