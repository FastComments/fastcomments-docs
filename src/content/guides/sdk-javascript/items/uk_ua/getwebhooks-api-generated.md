Перераховує вебхуки, налаштовані для орендаря, як рядки, керовані через панель, так і підписки API.

## Parameters

| Назва | Тип | Обов’язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| event | WebhookEventName | Ні |  |
| domain | string | Ні |  |
| source | WebhookSource | Ні |  |
| skip | number | Ні |  |

## Response

Returns: [`GetWebhooksResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetWebhooksResponse.ts)

## Example

[inline-code-attrs-start title = 'Приклад getWebhooks'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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