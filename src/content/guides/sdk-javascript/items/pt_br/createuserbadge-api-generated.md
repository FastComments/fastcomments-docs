---
## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| createUserBadgeParams | CreateUserBadgeParams | Sim |  |

## Resposta

Retorna: [`CreateUserBadge200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateUserBadge200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de createUserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_84f3b2';
const createUserBadgeParams: CreateUserBadgeParams = {
  name: 'Top Contributor',
  slug: 'top-contributor',
  imageUrl: 'https://assets.fastcomments.com/badges/top-contributor.png',
  description: 'Awarded for 100 helpful comments',
  active: true,
  criteria: { commentsCount: 100 }, // critérios opcionais
  displayOrder: 10,
  metadata: { featured: true } // metadados opcionais
};
const result: CreateUserBadge200Response = await createUserBadge(tenantId, createUserBadgeParams);
console.log(result);
[inline-code-end]

---