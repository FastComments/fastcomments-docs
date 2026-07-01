## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | No |  |
| badgeId | string | No |  |
| type | number | No |  |
| displayedOnComments | boolean | No |  |
| limit | number | No |  |
| skip | number | No |  |

## Yanıt

Döndürür: [`GetUserBadgesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadgesResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getUserBadges Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function example() {
  const tenantId: string = "tenant-01";
  const userId: string = "user-42";
  const badgeId: string = "badge-gold";
  const type: number = 1;
  const displayedOnComments: boolean = true;
  const limit: number = 10;
  const skip: number = 5;

  const fullResult: GetUserBadgesResponse = await getUserBadges(
    tenantId,
    userId,
    badgeId,
    type,
    displayedOnComments,
    limit,
    skip
  );

  const minimalResult: GetUserBadgesResponse = await getUserBadges(tenantId);
}
example();
[inline-code-end]