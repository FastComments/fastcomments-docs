## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-------------|
| tenantId | string | Sim |  |
| id | string | Sim |  |
| sendEmail | string | Não |  |

## Resposta

Retorna: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de deleteModerator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f8b7c6d';
const id: string = 'mod_4a3e11ec9d1f0242ac120003';
const sendEmail: string = 'true';
const result: FlagCommentPublic200Response = await deleteModerator(tenantId, id, sendEmail);
[inline-code-end]

---