## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| badgeId | string | Ja |  |
| userId | string | Nej |  |
| commentId | string | Nej |  |
| broadcastId | string | Nej |  |
| sso | string | Nej |  |

## Svar

Returnerer: [`RemoveUserBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/RemoveUserBadgeResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'putRemoveBadge Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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