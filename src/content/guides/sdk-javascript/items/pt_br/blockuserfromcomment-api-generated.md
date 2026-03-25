## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| tenantId | string | Sim |  |
| id | string | Sim |  |
| blockFromCommentParams | BlockFromCommentParams | Sim |  |
| userId | string | Não |  |
| anonUserId | string | Não |  |

## Resposta

Retorna: [`BlockFromCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BlockFromCommentPublic200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de blockUserFromComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const id: string = "comment_67890";
const blockFromCommentParams: BlockFromCommentParams = {
  reason: "Repeated abusive language",
  blockDurationHours: 168,
  blockReplies: true,
  notifyAuthor: true
};
const userId: string = "user_abc123";
const anonUserId: string = "anon_xyz789";
const result: BlockFromCommentPublic200Response = await blockUserFromComment(tenantId, id, blockFromCommentParams, userId, anonUserId);
[inline-code-end]

---