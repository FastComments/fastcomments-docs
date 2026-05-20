## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| userId | string | No |  |

## Response

Returns: [`GetTicket200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTicket200Response.ts)

## Example

[inline-code-attrs-start title = 'getTicket Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp';
const ticketId: string = 'TCKT-202605-042';
const userId: string = 'user-789';
const ticketResponse: GetTicket200Response = await getTicket(tenantId, ticketId);
const ticketResponseForUser: GetTicket200Response = await getTicket(tenantId, ticketId, userId);
[inline-code-end]
