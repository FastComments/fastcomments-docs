## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| badgeId | string | Да |  |
| userId | string | Не |  |
| commentId | string | Не |  |
| broadcastId | string | Не |  |
| sso | string | Не |  |

## Одговор

Враћа: [`RemoveUserBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/RemoveUserBadgeResponse.ts)

## Пример

[inline-code-attrs-start title = 'putRemoveBadge Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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