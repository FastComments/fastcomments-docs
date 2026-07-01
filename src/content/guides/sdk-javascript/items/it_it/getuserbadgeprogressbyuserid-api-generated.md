## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| userId | string | Sì |  |

## Risposta

Restituisce: [`GetUserBadgeProgressByUserIdResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadgeProgressByUserIdResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio getUserBadgeProgressByUserId'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const userId: string = "user-12345";

  const badgeProgress: GetUserBadgeProgressByUserIdResponse = await getUserBadgeProgressByUserId(tenantId, userId);
  console.log(badgeProgress);
})();
[inline-code-end]