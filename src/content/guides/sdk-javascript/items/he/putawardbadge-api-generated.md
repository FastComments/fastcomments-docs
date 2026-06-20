## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| badgeId | string | כן |  |
| userId | string | לא |  |
| commentId | string | לא |  |
| broadcastId | string | לא |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AwardUserBadgeResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת putAwardBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const badgeId: string = 'gold-medal-2023';
const userId: string = 'usr_100234';
const commentId: string = 'c_78910';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.fakePayload.signature';
const response: AwardUserBadgeResponse = await putAwardBadge(badgeId, userId, commentId, undefined, sso);
[inline-code-end]