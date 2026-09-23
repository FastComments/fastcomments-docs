Подписывает URL на событие комментария (REST‑hook подписка). Подписка того же URL на то же
событие и домен снова возвращает существующую подписку. Доставки подписаны HMAC, см.
руководство по веб‑хукам; устаревший заголовок `token` никогда не отправляется в подписки API.

## Parameters

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createWebhookParams | CreateWebhookParams | Да |  |

## Response

Возвращает: [`CreateWebhookResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateWebhookResponse.ts)

## Example

[inline-code-attrs-start title = 'Пример createWebhook'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9f8b7c6d";

const webhookParams: CreateWebhookParams = {
  event: "comment.created" as WebhookEventName,
  method: "POST" as WebhookHTTPMethod,
  url: "https://myapp.example.com/webhook",
  source: "comment" as WebhookSource,
  // необязательный секрет для проверки полезной нагрузки
  secret: "s3cr3tK3y123",
};

const result: CreateWebhookResponse = await createWebhook(tenantId, webhookParams);

const webhookId: string = result.webhook.id;
[inline-code-end]

---