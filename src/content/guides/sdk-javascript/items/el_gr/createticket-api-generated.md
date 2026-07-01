## Parameters

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| userId | string | Ναι |  |
| createTicketBody | CreateTicketBody | Ναι |  |

## Response

Επιστρέφει: [`CreateTicketResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTicketResponse1.ts)

## Example

[inline-code-attrs-start title = 'createTicket Παράδειγμα'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const userId: string = "user_98765";

const ticketBody: CreateTicketBody = {
  subject: "Issue with payment processing"
  // description?: string είναι προαιρετικό και παραλείπεται
};

const response: CreateTicketResponse1 = await createTicket(tenantId, userId, ticketBody);
// Παράδειγμα χρήσης ενός προαιρετικού πεδίου από την απόκριση
// console.log(response.ticket?.id);
[inline-code-end]