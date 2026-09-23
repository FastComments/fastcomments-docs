## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| commentId | string | Sim |  |
| approved | boolean | Não |  |
| broadcastId | string | Não |  |
| sso | string | Não |  |

## Resposta

Retorna: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SetCommentApprovedResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de postSetCommentApprovalStatus'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_001";
  const commentId: string = "comment_123";
  const approved: boolean = false;
  const broadcastId: string = "broadcast_456";
  const sso: string = "sso_789";

  const result: SetCommentApprovedResponse = await postSetCommentApprovalStatus(
    tenantId,
    commentId,
    approved,
    broadcastId,
    sso
  );

  const minimalResult: SetCommentApprovedResponse = await postSetCommentApprovalStatus(
    tenantId,
    commentId
  );
})();
[inline-code-end]