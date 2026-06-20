## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| badgesUserId | string | Nein |  |
| commentId | string | Nein |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserManualBadgesResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für getManualBadgesForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const badgesUserId: string = 'user_83b2f4';
const commentId: string = 'comment_9a1c7';
const ssoToken: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.exampleSignature';

const userBadges: GetUserManualBadgesResponse = await getManualBadgesForUser(badgesUserId);
const commentBadges: GetUserManualBadgesResponse = await getManualBadgesForUser(badgesUserId, commentId, ssoToken);
[inline-code-end]

---