## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| commentId | string | Sim |  |
| sso | string | Não |  |

## Resposta

Retorna: [`GetBannedUsersFromCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBannedUsersFromCommentResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getBanUsersFromComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c";
const commentId: string = "comment_a1b2c3";
const ssoToken: string = "sso_abc123";

const bannedUsers: GetBannedUsersFromCommentResponse = await getBanUsersFromComment(tenantId, commentId, ssoToken);
const bannedUsersNoSso: GetBannedUsersFromCommentResponse = await getBanUsersFromComment(tenantId, commentId);
[inline-code-end]