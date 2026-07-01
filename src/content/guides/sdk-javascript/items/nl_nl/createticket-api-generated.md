## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| userId | string | Ja |  |
| createTicketBody | CreateTicketBody | Ja |  |

## Respons

Retourneert: [`CreateTicketResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTicketResponse1.ts)

## Voorbeeld

[inline-code-attrs-start title = 'createTicket Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const userId: string = "user_98765";

const ticketBody: CreateTicketBody = {
  subject: "Issue with payment processing"
  // description?: string is optioneel en weggelaten
};

const response: CreateTicketResponse1 = await createTicket(tenantId, userId, ticketBody);
// Voorbeeld van het gebruiken van een optioneel veld uit de respons
// console.log(response.ticket?.id);
[inline-code-end]

---