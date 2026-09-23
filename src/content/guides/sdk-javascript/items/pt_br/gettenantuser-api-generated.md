## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| id | string | Sim |  |

## Resposta

Retorna: [`GetTenantUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUserResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
    const tenantId: string = "acme-tenant-001";
    const userId: string = "user-12345";
    const response: GetTenantUserResponse = await getTenantUser(tenantId, userId);
    console.log(response);
})();
[inline-code-end]