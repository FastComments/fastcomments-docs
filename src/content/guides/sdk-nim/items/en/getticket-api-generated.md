## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| userId | string | No |  |

## Response

Returns: [`Option[GetTicketResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_ticket_response.nim)

## Example

[inline-code-attrs-start title = 'getTicket Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTicket(tenantId = "my-tenant-123", id = "ticket-456", userId = "user-789")
if response.isSome:
  let ticket = response.get()
  echo "Got ticket:", ticket
else:
  echo "No ticket returned; HTTP response:", httpResponse
[inline-code-end]
