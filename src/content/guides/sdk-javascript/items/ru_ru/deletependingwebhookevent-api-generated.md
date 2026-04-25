## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |

## Ответ

Возвращает: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример deletePendingWebhookEvent'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f3b2a";
const webhookEventId: string = "wh_evt_9a8c7d1234";
const dryRun: boolean | undefined = undefined; // пример необязательного флага (не обязателен для этого вызова)
const result: FlagCommentPublic200Response = await deletePendingWebhookEvent(tenantId, webhookEventId);
[inline-code-end]