## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| badgeId | string | 예 |  |
| userId | string | 아니오 |  |
| commentId | string | 아니오 |  |
| broadcastId | string | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`RemoveUserBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/RemoveUserBadgeResponse.ts)

## 예시

[inline-code-attrs-start title = 'putRemoveBadge 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoRemoveBadge() {
  const tenantId: string = "tenant_42";
  const badgeId: string = "badge_gold";
  const userId: string = "user_1001";
  const commentId: string = "comment_20230915";
  const broadcastId: string = "broadcast_live_01";
  const sso: string = "sso_abcdef123456";

  const result: RemoveUserBadgeResponse = await putRemoveBadge(
    tenantId,
    badgeId,
    userId,
    commentId,
    broadcastId,
    sso
  );

  console.log(result);
}
[inline-code-end]