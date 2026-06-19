## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| skip | number | Non |  |

## Réponse

Retourne : [`GetTenantUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUsersResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getTenantUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_7f3b2a9c';
  const skip: number = 20; // paramètre optionnel démontré
  const result: GetTenantUsersResponse = await getTenantUsers(tenantId, skip);
  console.log(result);
})();
[inline-code-end]

---