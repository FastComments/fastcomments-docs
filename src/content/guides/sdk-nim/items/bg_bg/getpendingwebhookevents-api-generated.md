## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| options | GetPendingWebhookEventsOptions | Не |  |

## Отговор

Връща: [`Option[GetPendingWebhookEventsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_pending_webhook_events_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример за getPendingWebhookEvents'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResp, httpResp) = client.getPendingWebhookEvents(
  tenantId = "my-tenant-123",
  options = GetPendingWebhookEventsOptions()
)

if maybeResp.isSome:
  let resp = maybeResp.get()
[inline-code-end]