Підписує URL на подію коментаря (REST hook subscribe). Підписка того самого URL на ту ж подію та домен знову повертає існуючу підписку. Доставки підписані HMAC, дивіться посібник з вебхуків; застарілий заголовок `token` ніколи не надсилається до підписок API.

## Parameters

| Назва | Тип | Обов’язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| createWebhookParams | CreateWebhookParams | Так |  |

## Response

Returns: [`CreateWebhookResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateWebhookResponse.ts)

## Example

[inline-code-attrs-start title = 'createWebhook Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c6d";

const webhookParams: CreateWebhookParams = {
  event: "comment.created" as WebhookEventName,
  method: "POST" as WebhookHTTPMethod,
  url: "https://myapp.example.com/webhook",
  source: "comment" as WebhookSource,
  // optional secret for payload verification
  secret: "s3cr3tK3y123",
};

const result: CreateWebhookResponse = await createWebhook(tenantId, webhookParams);

const webhookId: string = result.webhook.id;
[inline-code-end]