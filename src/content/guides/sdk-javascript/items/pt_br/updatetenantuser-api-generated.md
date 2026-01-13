## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| id | string | Sim |  |
| updateTenantUserBody | UpdateTenantUserBody | Sim |  |
| updateComments | string | Não |  |

## Resposta

Retorna: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de updateTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_84f3b2';
const id: string = 'user_7a9d1c';
const updateComments: string = 'Promoted to moderator and updated contact email';
const updateTenantUserBody: UpdateTenantUserBody = {
  email: 'jane.doe+mod@example.com',
  displayName: 'Jane D.',
  roles: ['moderator'],
  isBanned: false,
  metadata: { department: 'community' }
};
const result: FlagCommentPublic200Response = await updateTenantUser(tenantId, id, updateTenantUserBody, updateComments);
[inline-code-end]

---