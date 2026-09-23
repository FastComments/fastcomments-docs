## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | כן |  |
| userId | string | לא |  |
| anonUserId | string | לא |  |

## תגובה

מחזיר: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת flagComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const commentId: string = "comment-20230915-001";
  const userId: string = "user-42";
  const anonUserId: string = "anon-abc123";

  const response: FlagCommentResponse = await flagComment(tenantId, commentId, userId, anonUserId);
  console.log(response);
})();
[inline-code-end]

---