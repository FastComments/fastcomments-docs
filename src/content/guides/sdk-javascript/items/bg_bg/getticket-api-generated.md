## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| userId | string | Не |  |

## Отговор

Връща: [`GetTicketResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTicketResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getTicket'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchTickets() {
  const tenantId: string = "acme-corp";
  const ticketId: string = "ticket-12345";
  const userId: string = "user-9876";

  const ticketWithUser: GetTicketResponse = await getTicket(tenantId, ticketId, userId);
  const ticketWithoutUser: GetTicketResponse = await getTicket(tenantId, ticketId);
}
[inline-code-end]