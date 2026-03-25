## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenantId | string | Sim |  |
| id | string | Sim |  |
| contextUserId | string | Não |  |
| isLive | boolean | Não |  |

## Resposta

Retorna: [`DeleteComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteComment200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de deleteComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7b3f2a';
const commentId: string = 'cmt_8a1f4d2b';
const contextUserId: string = 'user_102';
const isLive: boolean = true;
const result: DeleteComment200Response = await deleteComment(tenantId, commentId, contextUserId, isLive);
[inline-code-end]

---