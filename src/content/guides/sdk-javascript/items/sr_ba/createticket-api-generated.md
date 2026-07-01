## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| userId | string | Da |  |
| createTicketBody | CreateTicketBody | Da |  |

## Odgovor

Vraća: [`CreateTicketResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTicketResponse1.ts)

## Primjer

[inline-code-attrs-start title = 'createTicket Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const userId: string = "user_98765";

const ticketBody: CreateTicketBody = {
  subject: "Issue with payment processing"
  // description?: string je opcionalan i izostavljen
};

const response: CreateTicketResponse1 = await createTicket(tenantId, userId, ticketBody);
// Primjer korištenja opcionalnog polja iz odgovora
// console.log(response.ticket?.id);
[inline-code-end]