## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Odpowiedź

Zwraca: [`GetTenantUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUserResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
    const tenantId: string = "acme-tenant-001";
    const userId: string = "user-12345";
    const response: GetTenantUserResponse = await getTenantUser(tenantId, userId);
    console.log(response);
})();
[inline-code-end]

---