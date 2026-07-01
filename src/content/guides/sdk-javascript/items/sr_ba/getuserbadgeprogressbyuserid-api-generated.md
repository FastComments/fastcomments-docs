## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| userId | string | Da |  |

## Odgovor

Vraća: [`GetUserBadgeProgressByUserIdResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadgeProgressByUserIdResponse.ts)

## Primjer

[inline-code-attrs-start title = 'getUserBadgeProgressByUserId Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const userId: string = "user-12345";

  const badgeProgress: GetUserBadgeProgressByUserIdResponse = await getUserBadgeProgressByUserId(tenantId, userId);
  console.log(badgeProgress);
})();
[inline-code-end]