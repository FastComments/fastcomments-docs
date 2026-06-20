## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| badgeId | string | はい |  |
| userId | string | いいえ |  |
| commentId | string | いいえ |  |
| broadcastId | string | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AwardUserBadgeResponse.ts)

## 例

[inline-code-attrs-start title = 'putAwardBadge の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const badgeId: string = 'gold-medal-2023';
const userId: string = 'usr_100234';
const commentId: string = 'c_78910';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.fakePayload.signature';
const response: AwardUserBadgeResponse = await putAwardBadge(badgeId, userId, commentId, undefined, sso);
[inline-code-end]

---