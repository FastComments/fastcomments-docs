## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|--------------|-----------|
| commentId | string | Sim |  |
| broadcastId | string | Não |  |
| tenantId | string | Não |  |
| sso | string | Não |  |

## Resposta

Returns: [`PostUnFlagCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostUnFlagCommentResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'postUnFlagComment Exemplo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async () => {
  const response: PostUnFlagCommentResponse = await postUnFlagComment(
    "cmt_12345",          // commentId
    "brd_67890",          // broadcastId (opcional)
    "tenant_abc",         // tenantId (opcional)
    "sso_user_token_789"  // sso (opcional)
  );
};
[inline-code-end]