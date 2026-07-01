## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| commentId | string | כן |  |
| includeEmail | boolean | לא |  |
| includeIP | boolean | לא |  |
| tenantId | string | לא |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`GetModerationCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerationCommentResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getModerationComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchCommentDetails() {
  // קבוצת פרמטרים מלאה
  const commentId: string = "cmt_12345abc";
  const includeEmail: boolean = true;
  const includeIP: boolean = false;
  const tenantId: string = "tenant_9876";
  const sso: string = "sso_token_xyz";

  const fullResult: GetModerationCommentResponse = await getModerationComment(
    commentId,
    includeEmail,
    includeIP,
    tenantId,
    sso
  );

  // קריאה מינימלית עם הפרמטר הדרוש בלבד
  const minimalResult: GetModerationCommentResponse = await getModerationComment("cmt_67890def");

  // השתמש בתוצאות לפי הצורך...
}
[inline-code-end]

---