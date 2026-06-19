## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| commentId | string | Нет |  |
| externalId | string | Нет |  |
| eventType | string | Нет |  |
| type | string | Нет |  |
| domain | string | Нет |  |
| attemptCountGT | number | Нет |  |
| skip | number | Нет |  |

## Ответ

Возвращает: [`GetPendingWebhookEventsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEventsResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример getPendingWebhookEvents'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---