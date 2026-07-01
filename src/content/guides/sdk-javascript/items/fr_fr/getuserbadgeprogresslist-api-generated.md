## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| userId | string | Non |  |
| limit | number | Non |  |
| skip | number | Non |  |

## Réponse

Retourne : [`GetUserBadgeProgressListResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadgeProgressListResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getUserBadgeProgressList'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchBadgeProgress() {
  const tenantId: string = "acme-corp";
  const userId: string = "john.doe@example.com";
  const limit: number = 10;
  const skip: number = 5;

  const fullList: GetUserBadgeProgressListResponse = await getUserBadgeProgressList(
    tenantId,
    userId,
    limit,
    skip
  );

  const simpleList: GetUserBadgeProgressListResponse = await getUserBadgeProgressList(tenantId);
}

fetchBadgeProgress();
[inline-code-end]