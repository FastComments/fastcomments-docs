## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | כן |  |
| userId | string | לא |  |
| anonUserId | string | לא |  |

## תגובה

מחזיר: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-flagComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme-corp_01';
const commentId: string = 'cmt_5f8d7a2b3c4e';
const anonUserId: string = 'anon_9c3a1f0e';
const response: FlagCommentResponse = await flagComment(tenantId, commentId, undefined, anonUserId);
[inline-code-end]

---