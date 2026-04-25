## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenantId | string | Sim |  |
| id | string | Sim |  |

## Resposta

Retorna: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de deletePendingWebhookEvent'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f3b2a";
const webhookEventId: string = "wh_evt_9a8c7d1234";
const dryRun: boolean | undefined = undefined; // exemplo de flag opcional (não exigido por esta chamada)
const result: FlagCommentPublic200Response = await deletePendingWebhookEvent(tenantId, webhookEventId);
[inline-code-end]

---