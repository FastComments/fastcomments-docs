## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| sso | string | No |  |

## תגובה

מחזיר: [`GetBannedUsersFromCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBannedUsersFromCommentResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'getBanUsersFromComment דוגמה'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c";
const commentId: string = "comment_a1b2c3";
const ssoToken: string = "sso_abc123";

const bannedUsers: GetBannedUsersFromCommentResponse = await getBanUsersFromComment(tenantId, commentId, ssoToken);
const bannedUsersNoSso: GetBannedUsersFromCommentResponse = await getBanUsersFromComment(tenantId, commentId);
[inline-code-end]