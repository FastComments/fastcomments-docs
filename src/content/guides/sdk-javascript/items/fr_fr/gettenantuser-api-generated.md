## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Réponse

Renvoie : [`GetTenantUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUserResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
    const tenantId: string = "acme-tenant-001";
    const userId: string = "user-12345";
    const response: GetTenantUserResponse = await getTenantUser(tenantId, userId);
    console.log(response);
})();
[inline-code-end]