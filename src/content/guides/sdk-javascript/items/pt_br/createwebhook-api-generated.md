Inscreve uma URL em um evento de comentário (inscrição de hook REST). Inscrever a mesma URL no mesmo evento e domínio novamente retorna a inscrição existente. As entregas são assinadas com HMAC, veja o guia de webhooks; o cabeçalho legado `token` nunca é enviado para inscrições de API.

## Parameters

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| createWebhookParams | CreateWebhookParams | Sim |  |

## Response

Retorna: [`CreateWebhookResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateWebhookResponse.ts)

## Example

[inline-code-attrs-start title = 'Exemplo createWebhook'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c6d";

const webhookParams: CreateWebhookParams = {
  event: "comment.created" as WebhookEventName,
  method: "POST" as WebhookHTTPMethod,
  url: "https://myapp.example.com/webhook",
  source: "comment" as WebhookSource,
  // segredo opcional para verificação da carga útil
  secret: "s3cr3tK3y123",
};

const result: CreateWebhookResponse = await createWebhook(tenantId, webhookParams);

const webhookId: string = result.webhook.id;
[inline-code-end]