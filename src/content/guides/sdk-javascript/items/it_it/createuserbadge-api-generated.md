## Parametri

| Name | Tipo | Richiesto | Descrizione |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createUserBadgeParams | CreateUserBadgeParams | Yes |  |

## Risposta

Restituisce: [`APICreateUserBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APICreateUserBadgeResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di createUserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_84a2c6b2';
  const createUserBadgeParams: CreateUserBadgeParams = {
    name: 'Early Supporter',
    description: 'Awarded to users who joined during the alpha launch',
    iconUrl: 'https://cdn.fastcomments.com/badges/early-supporter.png',
    criteria: 'Joined before 2021-06-01',
    isActive: true,
    notifyUsers: true // parametro opzionale
  };
  const result: APICreateUserBadgeResponse = await createUserBadge(tenantId, createUserBadgeParams);
  console.log(result);
})();
[inline-code-end]

---