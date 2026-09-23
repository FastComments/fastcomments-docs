## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| userId | string | No |  |

## Risposta

Restituisce: [`GetTicketResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTicketResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio getTicket'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchTickets() {
  const tenantId: string = "acme-corp";
  const ticketId: string = "ticket-12345";
  const userId: string = "user-9876";

  const ticketWithUser: GetTicketResponse = await getTicket(tenantId, ticketId, userId);
  const ticketWithoutUser: GetTicketResponse = await getTicket(tenantId, ticketId);
}
[inline-code-end]

---