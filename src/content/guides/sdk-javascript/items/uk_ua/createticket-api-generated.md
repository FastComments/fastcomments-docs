## Параметри

| Назва | Тип | Обов'язковий | Опис |
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
  // description?: string is optional and omitted  // опис?: string є необов'язковим і пропущений
};

const response: CreateTicketResponse1 = await createTicket(tenantId, userId, ticketBody);
// Example of using an optional field from the response  // Приклад використання необов'язкового поля з відповіді
// console.log(response.ticket?.id);
[inline-code-end]