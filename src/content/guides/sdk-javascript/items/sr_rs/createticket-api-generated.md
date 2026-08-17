## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| userId | string | Да |  |
| createTicketBody | CreateTicketBody | Да |  |

## Одговор

Враћа: [`CreateTicketResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTicketResponse1.ts)

## Пример

[inline-code-attrs-start title = 'createTicket пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const userId: string = "user_98765";

const ticketBody: CreateTicketBody = {
  subject: "Issue with payment processing"
  // description?: string је опционо и изостављено
};

const response: CreateTicketResponse1 = await createTicket(tenantId, userId, ticketBody);
// Пример коришћења опционо поље из одговора
// console.log(response.ticket?.id);
[inline-code-end]