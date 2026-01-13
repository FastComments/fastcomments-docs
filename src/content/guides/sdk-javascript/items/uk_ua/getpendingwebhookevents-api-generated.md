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
| skip | number | Ні |  |

## Response

Повертає: [`GetPendingWebhookEvents200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPendingWebhookEvents200Response.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getPendingWebhookEvents'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_78b2f1";
const commentId: string = "cmt_0042";
const eventType: string = "comment.created";
const domain: string = "blog.example.com";
const attemptCountGT: number = 1;
const skip: number = 0;

const pending: GetPendingWebhookEvents200Response = await getPendingWebhookEvents(
  tenantId,
  commentId,
  undefined, // externalId — зовнішній ідентифікатор
  eventType,
  undefined, // type — тип
  domain,
  attemptCountGT,
  skip
);
[inline-code-end]

---