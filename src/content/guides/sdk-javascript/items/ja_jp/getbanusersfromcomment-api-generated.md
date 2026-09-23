## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| commentId | string | はい |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`GetBannedUsersFromCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBannedUsersFromCommentResponse.ts)

## 例

[inline-code-attrs-start title = 'getBanUsersFromComment の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c";
const commentId: string = "comment_a1b2c3";
const ssoToken: string = "sso_abc123";

const bannedUsers: GetBannedUsersFromCommentResponse = await getBanUsersFromComment(tenantId, commentId, ssoToken);
const bannedUsersNoSso: GetBannedUsersFromCommentResponse = await getBanUsersFromComment(tenantId, commentId);
[inline-code-end]