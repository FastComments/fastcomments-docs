## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| badgeId | string | Yes |  |
| userId | string | No |  |
| commentId | string | No |  |
| broadcastId | string | No |  |
| sso | string | No |  |

## Odgovor

Vraća: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AwardUserBadgeResponse.ts)

## Primjer

[inline-code-attrs-start title = 'putAwardBadge Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---