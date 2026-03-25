## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | No |  |
| state | float64 | No |  |
| skip | float64 | No |  |
| limit | float64 | No |  |

## Response

Returns: [`Option[GetTickets_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tickets200response.nim)

## Example

[inline-code-attrs-start title = 'getTickets Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTickets(tenantId = "my-tenant-123", userId = "user-456", state = 1.0, skip = 0.0, limit = 50.0)
if response.isSome:
  let tickets = response.get()
  echo tickets
else:
  echo "No tickets returned"
[inline-code-end]
