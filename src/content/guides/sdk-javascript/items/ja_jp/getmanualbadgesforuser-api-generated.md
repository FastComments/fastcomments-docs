## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| badgesUserId | string | いいえ |  |
| commentId | string | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserManualBadgesResponse.ts)

## 例

[inline-code-attrs-start title = 'getManualBadgesForUser の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const badgesUserId: string = 'user_83b2f4';
const commentId: string = 'comment_9a1c7';
const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.exampleSignature';

const userBadges: GetUserManualBadgesResponse = await getManualBadgesForUser(badgesUserId);
const commentBadges: GetUserManualBadgesResponse = await getManualBadgesForUser(badgesUserId, commentId, ssoToken);
[inline-code-end]

---