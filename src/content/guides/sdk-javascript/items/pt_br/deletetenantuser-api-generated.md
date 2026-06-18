---
## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenantId | string | Sim |  |
| id | string | Sim |  |
| deleteComments | string | Não |  |
| commentDeleteMode | string | Não |  |

## Response

Retorna: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de deleteTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_8f3a2b1c4d";
const id: string = "user_62a4f9e0b7";
const deleteComments: string = "true";
const commentDeleteMode: string = "permanent";
const result: FlagCommentPublic200Response = await deleteTenantUser(tenantId, id, deleteComments, commentDeleteMode);
[inline-code-end]

---