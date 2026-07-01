---
## Parametre

| Navn | Type | Krævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| userId | string | Ja |  |

## Svar

Returnerer: [`GetUserBadgeProgressByUserIdResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadgeProgressByUserIdResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'getUserBadgeProgressByUserId Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const userId: string = "user-12345";

  const badgeProgress: GetUserBadgeProgressByUserIdResponse = await getUserBadgeProgressByUserId(tenantId, userId);
  console.log(badgeProgress);
})();
[inline-code-end]

---