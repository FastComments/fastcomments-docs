## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| userId | string | Не |  |
| state | float64 | Не |  |
| skip | float64 | Не |  |
| limit | float64 | Не |  |

## Отговор

Връща: [`Option[GetTickets_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tickets200response.nim)

## Пример

[inline-code-attrs-start title = 'Пример за getTickets'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTickets(tenantId = "my-tenant-123", userId = "user-456", state = 1.0, skip = 0.0, limit = 50.0)
if response.isSome:
  let tickets = response.get()
  echo tickets
else:
  echo "No tickets returned"
[inline-code-end]

---