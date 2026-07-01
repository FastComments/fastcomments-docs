## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|------------|---------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| userId | string | Nee |  |

## Respons

Retour: [`GetTicketResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTicketResponse1.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getTicket Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-inc";
const ticketId: string = "ticket-3421";
const userId: string = "alice.smith";

const ticketWithUser: GetTicketResponse1 = await getTicket(tenantId, ticketId, userId);
const ticketWithoutUser: GetTicketResponse1 = await getTicket(tenantId, ticketId);
[inline-code-end]