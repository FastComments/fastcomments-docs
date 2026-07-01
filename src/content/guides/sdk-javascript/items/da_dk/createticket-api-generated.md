## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| userId | string | Ja |  |
| createTicketBody | CreateTicketBody | Ja |  |

## Svar

Returnerer: [`CreateTicketResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTicketResponse1.ts)

## Eksempel

[inline-code-attrs-start title = 'createTicket Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const userId: string = "user_98765";

const ticketBody: CreateTicketBody = {
  subject: "Issue with payment processing"
  // description?: string er valgfri og udeladt
};

const response: CreateTicketResponse1 = await createTicket(tenantId, userId, ticketBody);
// Eksempel på brug af et valgfrit felt fra svaret
// console.log(response.ticket?.id);
[inline-code-end]