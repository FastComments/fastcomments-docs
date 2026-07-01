## Parameters

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| userId | string | Sì |  |
| createTicketBody | CreateTicketBody | Sì |  |

## Response

Restituisce: [`CreateTicketResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTicketResponse1.ts)

## Example

[inline-code-attrs-start title = 'Esempio createTicket'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const userId: string = "user_98765";

const ticketBody: CreateTicketBody = {
  subject: "Issue with payment processing"
  // description?: string è opzionale e omesso
};

const response: CreateTicketResponse1 = await createTicket(tenantId, userId, ticketBody);
// Esempio di utilizzo di un campo opzionale dalla risposta
// console.log(response.ticket?.id);
[inline-code-end]