## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| badgesUserId | string | Ні |  |
| commentId | string | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserManualBadgesResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getManualBadgesForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const badgesUserId: string = 'user_83b2f4';
const commentId: string = 'comment_9a1c7';
const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.exampleSignature';

const userBadges: GetUserManualBadgesResponse = await getManualBadgesForUser(badgesUserId);
const commentBadges: GetUserManualBadgesResponse = await getManualBadgesForUser(badgesUserId, commentId, ssoToken);
[inline-code-end]

---