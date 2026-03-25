## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Resposta

Retorna: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de deletePendingWebhookEvent'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_42f7c9b1';
  const id: string = 'pending_webhook_ev_8f3b9a2d';
  const reason?: string = undefined; // exemplo de parâmetro opcional (não exigido pela função)
  const result: FlagCommentPublic200Response = await deletePendingWebhookEvent(tenantId, id);
  console.log(result);
})();
[inline-code-end]

---