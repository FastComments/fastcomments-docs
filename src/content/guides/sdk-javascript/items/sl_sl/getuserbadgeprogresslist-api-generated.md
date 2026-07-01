## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| userId | string | Ne |  |
| limit | number | Ne |  |
| skip | number | Ne |  |

## Odziv

Vrne: [`GetUserBadgeProgressListResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadgeProgressListResponse.ts)

## Primer

[inline-code-attrs-start title = 'getUserBadgeProgressList Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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