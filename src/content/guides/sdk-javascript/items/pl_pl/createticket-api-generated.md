## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | Yes |  |
| createTicketBody | CreateTicketBody | Yes |  |

## Odpowiedź

Zwraca: [`CreateTicketResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTicketResponse1.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład createTicket'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const userId: string = "user_98765";

const ticketBody: CreateTicketBody = {
  subject: "Issue with payment processing"
  // description?: string jest opcjonalny i pominięty
};

const response: CreateTicketResponse1 = await createTicket(tenantId, userId, ticketBody);
// Przykład użycia opcjonalnego pola z odpowiedzi
// console.log(response.ticket?.id);
[inline-code-end]