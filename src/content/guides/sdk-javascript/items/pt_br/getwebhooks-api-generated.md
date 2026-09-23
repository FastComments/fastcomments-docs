Lista os webhooks configurados para o tenant, tanto linhas gerenciadas pelo painel quanto assinaturas de API.

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| event | WebhookEventName | Não |  |
| domain | string | Não |  |
| source | WebhookSource | Não |  |
| skip | number | Não |  |

## Resposta

Retorna: [`GetWebhooksResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetWebhooksResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getWebhooks'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_42';
  const event: WebhookEventName = 'comment.created';
  const domain: string = 'forum.example.org';
  const source: WebhookSource = 'admin';
  const skip: number = 0;

  const fullResponse: GetWebhooksResponse = await getWebhooks(tenantId, event, domain, source, skip);
  const minimalResponse: GetWebhooksResponse = await getWebhooks(tenantId);
})();
[inline-code-end]

---