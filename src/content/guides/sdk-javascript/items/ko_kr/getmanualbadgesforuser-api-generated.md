## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| badgesUserId | string | 아니요 |  |
| commentId | string | 아니요 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserManualBadgesResponse.ts)

## 예제

[inline-code-attrs-start title = 'getManualBadgesForUser 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const badgesUserId: string = 'user_83b2f4';
const commentId: string = 'comment_9a1c7';
const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.exampleSignature';

const userBadges: GetUserManualBadgesResponse = await getManualBadgesForUser(badgesUserId);
const commentBadges: GetUserManualBadgesResponse = await getManualBadgesForUser(badgesUserId, commentId, ssoToken);
[inline-code-end]

---