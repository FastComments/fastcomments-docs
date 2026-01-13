---
## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| commentId | string | Не |  |
| externalId | string | Не |  |
| eventType | string | Не |  |
| type | string | Не |  |
| domain | string | Не |  |
| attemptCountGT | number | Не |  |

## Одговор

Враћа: [`GetPendingWebhookEventCount200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEventCount200Response.ts)

## Пример

[inline-code-attrs-start title = 'getPendingWebhookEventCount Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_9c3b2b';
  const commentId: string = 'cmt_f4a1b2';
  const externalId: string = 'ext-789';
  const eventType: string = 'comment.created';
  const type: string = 'delivery';
  const domain: string = 'app.example.com';
  const attemptCountGT: number = 2;

  const result: GetPendingWebhookEventCount200Response = await getPendingWebhookEventCount(
    tenantId,
    commentId,
    externalId,
    eventType,
    type,
    domain,
    attemptCountGT
  );

  console.log(result);
})();
[inline-code-end]

---