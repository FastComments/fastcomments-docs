## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| badgeId | string | はい |  |
| userId | string | いいえ |  |
| commentId | string | いいえ |  |
| broadcastId | string | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`RemoveUserBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/RemoveUserBadgeResponse.ts)

## 例

[inline-code-attrs-start title = 'putRemoveBadge の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const badgeId: string = 'badge_7392';
  const userId: string = 'user_1284';
  const commentId: string = 'cmt_5583';
  const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ1c2VyXzEyODQifQ.signature';
  const response: RemoveUserBadgeResponse = await putRemoveBadge(badgeId, userId, commentId, undefined, sso);
  console.log(response);
})();
[inline-code-end]

---