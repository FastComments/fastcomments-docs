## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| userId | string | No |  |
| limit | number | No |  |
| skip | number | No |  |

## Odpowiedź

Zwraca: [`GetUserBadgeProgressListResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadgeProgressListResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getUserBadgeProgressList'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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