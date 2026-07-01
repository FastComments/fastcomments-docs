## Parameter

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | Yes |  |
| createTicketBody | CreateTicketBody | Yes |  |

## Antwort

Rückgabe: [`CreateTicketResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTicketResponse1.ts)

## Beispiel

[inline-code-attrs-start title = 'createTicket Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const userId: string = "user_98765";

const ticketBody: CreateTicketBody = {
  subject: "Issue with payment processing"
  // description?: string ist optional und weggelassen
};

const response: CreateTicketResponse1 = await createTicket(tenantId, userId, ticketBody);
// Beispiel für die Verwendung eines optionalen Feldes aus der Antwort
// console.log(response.ticket?.id);
[inline-code-end]