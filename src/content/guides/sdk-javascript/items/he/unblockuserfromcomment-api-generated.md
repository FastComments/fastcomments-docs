## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | כן |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | כן |  |
| userId | string | לא |  |
| anonUserId | string | לא |  |

## תגובה

מחזיר: [`UnBlockCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnBlockCommentPublic200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה של unBlockUserFromComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_87f3e1';
const id: string = 'comment_9b2a4f';
const unBlockFromCommentParams: UnBlockFromCommentParams = {
  reason: 'Reviewed by moderation team — reinstated',
  moderatorId: 'mod_21',
  unblockedAt: new Date().toISOString()
};
const userId: string = 'user_42';
const anonUserId: string = 'anon_e7f9';
const result: UnBlockCommentPublic200Response = await unBlockUserFromComment(tenantId, id, unBlockFromCommentParams, userId, anonUserId);
[inline-code-end]

---