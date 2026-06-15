## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| commentId | string | כן |  |
| dir | number | כן |  |
| sso | string | לא |  |

## תשובה

מחזיר: [`GetCommentVoteUserNames200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentVoteUserNames200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getCommentVoteUserNames'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_67890";
const commentId: string = "comment_abc123";
const dir: number = 1;
const ssoToken: string = "sso-eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9";

const responseWithoutSSO: GetCommentVoteUserNames200Response = await getCommentVoteUserNames(tenantId, commentId, dir);
const responseWithSSO: GetCommentVoteUserNames200Response = await getCommentVoteUserNames(tenantId, commentId, dir, ssoToken);
[inline-code-end]

---