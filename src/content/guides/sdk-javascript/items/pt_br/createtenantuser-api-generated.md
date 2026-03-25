## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenantId | string | Sim |  |
| createTenantUserBody | CreateTenantUserBody | Sim |  |

## Resposta

Retorna: [`CreateTenantUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantUser200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de createTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_6f4b2c';
const createTenantUserBody: CreateTenantUserBody = {
  email: 'sara.kim@example.com',
  displayName: 'Sara Kim',
  role: 'moderator',
  notifyOnMentions: true
};
const result: CreateTenantUser200Response = await createTenantUser(tenantId, createTenantUserBody);
[inline-code-end]

---