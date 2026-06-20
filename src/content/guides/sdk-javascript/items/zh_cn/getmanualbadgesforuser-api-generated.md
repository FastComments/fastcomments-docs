---
## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| badgesUserId | string | 否 |  |
| commentId | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回：[`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserManualBadgesResponse.ts)

## 示例

[inline-code-attrs-start title = 'getManualBadgesForUser 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const badgesUserId: string = 'user_83b2f4';
const commentId: string = 'comment_9a1c7';
const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.exampleSignature';

const userBadges: GetUserManualBadgesResponse = await getManualBadgesForUser(badgesUserId);
const commentBadges: GetUserManualBadgesResponse = await getManualBadgesForUser(badgesUserId, commentId, ssoToken);
[inline-code-end]

---