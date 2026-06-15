## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |

## Risposta

Restituisce: [`GetUserBadge200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadge200Response.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di getUserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_6b8f2a1c';
  const id: string = 'badge_9f3d4b2a';
  const response: GetUserBadge200Response = await getUserBadge(tenantId, id);
  const badge: UserBadge | undefined = response.userBadge;
  const badgeName: string | undefined = badge?.name;
  console.log('Retrieved badge name:', badgeName);
})();
[inline-code-end]

---