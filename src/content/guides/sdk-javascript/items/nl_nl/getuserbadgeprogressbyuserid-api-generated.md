## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|--------------|
| tenantId | string | Ja |  |
| userId | string | Ja |  |

## Respons

Retourneert: [`GetUserBadgeProgressByUserIdResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadgeProgressByUserIdResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'Voorbeeld getUserBadgeProgressByUserId'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const userId: string = "user-12345";

  const badgeProgress: GetUserBadgeProgressByUserIdResponse = await getUserBadgeProgressByUserId(tenantId, userId);
  console.log(badgeProgress);
})();
[inline-code-end]