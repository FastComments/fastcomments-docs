## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | Yes |  |
| createTicketBody | CreateTicketBody | Yes |  |

## Отговор

Връща: [`CreateTicketResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTicketResponse1.ts)

## Пример

[inline-code-attrs-start title = 'createTicket Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const userId: string = "user_98765";

const ticketBody: CreateTicketBody = {
  subject: "Issue with payment processing"
  // description?: string е незадължително и е пропуснато
};

const response: CreateTicketResponse1 = await createTicket(tenantId, userId, ticketBody);
// Пример за използване на незадължително поле от отговора
// console.log(response.ticket?.id);
[inline-code-end]