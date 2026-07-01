## Параметри

| Назва | Тип | Обов’язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | Yes |  |
| createTicketBody | CreateTicketBody | Yes |  |

## Відповідь

Повертає: [`CreateTicketResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTicketResponse1.ts)

## Приклад

[inline-code-attrs-start title = 'createTicket Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const userId: string = "user_98765";

const ticketBody: CreateTicketBody = {
  subject: "Issue with payment processing"
  // description?: string є необов’язковим і опущено
};

const response: CreateTicketResponse1 = await createTicket(tenantId, userId, ticketBody);
// Приклад використання необов’язкового поля у відповіді
// console.log(response.ticket?.id);
[inline-code-end]