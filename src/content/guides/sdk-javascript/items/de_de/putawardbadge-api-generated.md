## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| badgeId | string | Ja |  |
| userId | string | Nein |  |
| commentId | string | Nein |  |
| broadcastId | string | Nein |  |
| sso | string | Nein |  |

## Antwort

Rückgabe: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AwardUserBadgeResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'putAwardBadge Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function awardBadgeExample(): Promise<void> {
  const tenantId: string = "tenant_42";
  const badgeId: string = "badge_superstar";
  const userId: string = "user_1001";
  const commentId: string = "comment_2023";

  const result: AwardUserBadgeResponse = await putAwardBadge(
    tenantId,
    badgeId,
    userId,
    commentId
  );

  console.log(result);
}
[inline-code-end]