## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| commentId | string | Sim |  |
| includeEmail | boolean | Não |  |
| includeIP | boolean | Não |  |
| sso | string | Não |  |

## Resposta

Retorna: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPICommentResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getModerationComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchComments() {
  const tenantId: string = "acme-corp";
  const commentId: string = "cmt_1234567890";

  // Chamada com apenas os parâmetros obrigatórios
  const basicResponse: ModerationAPICommentResponse = await getModerationComment(tenantId, commentId);

  // Chamada com parâmetros opcionais
  const includeEmail: boolean = true;
  const includeIP: boolean = false;
  const sso: string = "sso-token-abc123";
  const detailedResponse: ModerationAPICommentResponse = await getModerationComment(
    tenantId,
    commentId,
    includeEmail,
    includeIP,
    sso
  );
}
[inline-code-end]