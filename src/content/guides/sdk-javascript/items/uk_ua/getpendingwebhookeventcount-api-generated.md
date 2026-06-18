## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| commentId | string | Ні |  |
| externalId | string | Ні |  |
| eventType | string | Ні |  |
| type | string | Ні |  |
| domain | string | Ні |  |
| attemptCountGT | number | Ні |  |

## Відповідь

Повертає: [`GetPendingWebhookEventCount200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEventCount200Response.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад використання getPendingWebhookEventCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f8b3b';
const commentId: string = 'cmt_1a2b3c';
const eventType: string = 'comment.created';
const domain: string = 'news-site.com';
const attemptCountGT: number = 2;

const result: GetPendingWebhookEventCount200Response = await getPendingWebhookEventCount(
  tenantId,
  commentId,
  undefined, // externalId omitted
  eventType,
  undefined, // type omitted
  domain,
  attemptCountGT
);
[inline-code-end]

---