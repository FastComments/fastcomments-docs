## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| commentId | string | Sim |  |
| reviewed | boolean | Não |  |
| broadcastId | string | Não |  |
| sso | string | Não |  |

## Resposta

Retorna: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'postSetCommentReviewStatus Exemplo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const commentId: string = "cmt_98765";
const reviewed: boolean = true;
const broadcastId: string = "brd_001";
const sso: string = "sso_token_abc";

const result: APIEmptyResponse = await postSetCommentReviewStatus(
  tenantId,
  commentId,
  reviewed,
  broadcastId,
  sso
);
[inline-code-end]