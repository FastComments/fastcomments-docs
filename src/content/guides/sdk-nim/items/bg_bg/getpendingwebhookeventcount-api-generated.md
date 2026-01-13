## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| commentId | string | Да |  |
| externalId | string | Не |  |
| eventType | string | Не |  |
| domain | string | Не |  |
| attemptCountGT | float64 | Не |  |

## Отговор

Връща: [`Option[GetPendingWebhookEventCount_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_pending_webhook_event_count200response.nim)

## Пример

[inline-code-attrs-start title = 'Пример за getPendingWebhookEventCount'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPendingWebhookEventCount(
  tenantId = "my-tenant-123",
  commentId = "cmt-4567",
  externalId = "",
  eventType = "",
  domain = "",
  attemptCountGT = 0.0
)
if response.isSome:
  let pending = response.get()
  echo "Received pending webhook event count response: ", $pending
else:
  echo "No pending webhook event count returned, HTTP response: ", $httpResponse
[inline-code-end]

---