Претплаћује URL на догађај коментара (REST hook претплата). Поновна претплата истог URL-а на исти
догађај и домен враћа постојећу претплату. Испоруке су потписане HMAC-ом, погледајте водич за
webhooks; застарели `token` заглавље се никада не шаље API претплатама.

## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createWebhookParams | CreateWebhookParams | Да |  |

## Одговор

Враћа: [`CreateWebhookResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateWebhookResponse.ts)

## Пример

[inline-code-attrs-start title = 'createWebhook пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c6d";

const webhookParams: CreateWebhookParams = {
  event: "comment.created" as WebhookEventName,
  method: "POST" as WebhookHTTPMethod,
  url: "https://myapp.example.com/webhook",
  source: "comment" as WebhookSource,
  // опциона тајна за верификацију payload-а
  secret: "s3cr3tK3y123",
};

const result: CreateWebhookResponse = await createWebhook(tenantId, webhookParams);

const webhookId: string = result.webhook.id;
[inline-code-end]