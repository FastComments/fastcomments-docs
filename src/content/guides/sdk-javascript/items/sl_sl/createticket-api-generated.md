## Parameters

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| userId | string | Da |  |
| createTicketBody | CreateTicketBody | Da |  |

## Response

Vrne: [`CreateTicketResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTicketResponse1.ts)

## Example

[inline-code-attrs-start title = 'createTicket Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const userId: string = "user_98765";

const ticketBody: CreateTicketBody = {
  subject: "Issue with payment processing"
  // description?: string je neobvezen in je izpuščen
};

const response: CreateTicketResponse1 = await createTicket(tenantId, userId, ticketBody);
// Primer uporabe neobveznega polja iz odgovora
// console.log(response.ticket?.id);
[inline-code-end]