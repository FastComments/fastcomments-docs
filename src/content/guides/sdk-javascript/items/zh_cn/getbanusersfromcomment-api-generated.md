## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| sso | string | 否 |  |

## 响应

返回: [`GetBannedUsersFromCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBannedUsersFromCommentResponse.ts)

## 示例

[inline-code-attrs-start title = 'getBanUsersFromComment 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c";
const commentId: string = "comment_a1b2c3";
const ssoToken: string = "sso_abc123";

const bannedUsers: GetBannedUsersFromCommentResponse = await getBanUsersFromComment(tenantId, commentId, ssoToken);
const bannedUsersNoSso: GetBannedUsersFromCommentResponse = await getBanUsersFromComment(tenantId, commentId);
[inline-code-end]