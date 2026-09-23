## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| includeEmail | boolean | No |  |
| includeIP | boolean | No |  |
| sso | string | No |  |

## תגובה

מחזיר: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPICommentResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getModerationComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchComments() {
  const tenantId: string = "acme-corp";
  const commentId: string = "cmt_1234567890";

  // קריאה רק עם הפרמטרים הדרושים
  const basicResponse: ModerationAPICommentResponse = await getModerationComment(tenantId, commentId);

  // קריאה עם פרמטרים אופציונליים
  const includeEmail: boolean = true;
  const includeIP: boolean = false;
  const sso: string = "sso-token-abc123";
  const detailedResponse: ModerationAPICommentResponse = await getModerationComment(
    tenantId,
    commentId,
    includeEmail,
    includeIP,
    sso
  );
}
[inline-code-end]