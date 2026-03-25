## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| userId | string | No |  |

## Одговор

Враћа: [`Option[GetTicket_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_ticket200response.nim)

## Пример

[inline-code-attrs-start title = 'getTicket пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTicket(tenantId = "my-tenant-123", id = "", userId = "")
if response.isSome:
  let ticket = response.get()
  discard ticket
[inline-code-end]