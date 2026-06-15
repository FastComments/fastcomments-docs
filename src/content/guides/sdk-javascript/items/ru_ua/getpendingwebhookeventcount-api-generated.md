## Параметры

| Name | Type | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| commentId | string | Нет |  |
| externalId | string | Нет |  |
| eventType | string | Нет |  |
| type | string | Нет |  |
| domain | string | Нет |  |
| attemptCountGT | number | Нет |  |

## Ответ

Возвращает: [`GetPendingWebhookEventCount200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEventCount200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример getPendingWebhookEventCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f8b3b';
const commentId: string = 'cmt_1a2b3c';
const eventType: string = 'comment.created';
const domain: string = 'news-site.com';
const attemptCountGT: number = 2;

const result: GetPendingWebhookEventCount200Response = await getPendingWebhookEventCount(
  tenantId,
  commentId,
  undefined, // externalId опущен
  eventType,
  undefined, // type опущен
  domain,
  attemptCountGT
);
[inline-code-end]

---