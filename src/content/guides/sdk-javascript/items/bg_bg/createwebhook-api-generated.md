Абонира URL за събитие при коментар (REST hook subscribe). Повторно абониране на същия URL за същото
събитие и домейн връща съществуващия абонамент. Доставките са подписани с HMAC, вижте ръководството за
webhooks; legacy хедъра `token` никога не се изпраща към API абонаменти.

## Параметри

| Име | Тип | Задължителен | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createWebhookParams | CreateWebhookParams | Да |  |

## Отговор

Връща: [`CreateWebhookResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateWebhookResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за createWebhook'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c6d";

const webhookParams: CreateWebhookParams = {
  event: "comment.created" as WebhookEventName,
  method: "POST" as WebhookHTTPMethod,
  url: "https://myapp.example.com/webhook",
  source: "comment" as WebhookSource,
  // незадължителна тайна за проверка на payload
  secret: "s3cr3tK3y123",
};

const result: CreateWebhookResponse = await createWebhook(tenantId, webhookParams);

const webhookId: string = result.webhook.id;
[inline-code-end]