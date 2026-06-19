## Параметри

| Назив | Type | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| commentId | string | Не |  |
| externalId | string | Не |  |
| eventType | string | Не |  |
| type | string | Не |  |
| domain | string | Не |  |
| attemptCountGT | number | Не |  |
| skip | number | Не |  |

## Одговор

Враћа: [`GetPendingWebhookEventsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEventsResponse.ts)

## Пример

[inline-code-attrs-start title = 'getPendingWebhookEvents Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_a1b2c3';
const commentId: string = 'cmt_9f8e7d';
const eventType: string = 'comment.created';
const domain: string = 'comments.acme-corp.com';
const attemptCountGT: number = 2;
const skip: number = 5;

const result: GetPendingWebhookEventsResponse = await getPendingWebhookEvents(
  tenantId,
  commentId,
  undefined,
  eventType,
  undefined,
  domain,
  attemptCountGT,
  skip
);
[inline-code-end]