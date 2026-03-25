## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| id | string | Sim |  |
| userId | string | Não |  |
| anonUserId | string | Não |  |

## Resposta

Retorna: [`FlagComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagComment200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de flagComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_001';
const id: string = 'comment_7f3a2b9e';
const userId: string = 'user_jdoe_1001';
const anonUserId: string = 'anon_3f2b_visitor';
const result: FlagComment200Response = await flagComment(tenantId, id, userId, anonUserId);
[inline-code-end]

---