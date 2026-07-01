## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | Yes |  |
| createTicketBody | CreateTicketBody | Yes |  |

## Ответ

Возвращает: [`CreateTicketResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTicketResponse1.ts)

## Пример

[inline-code-attrs-start title = 'Пример createTicket'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const userId: string = "user_98765";

const ticketBody: CreateTicketBody = {
  subject: "Issue with payment processing"
  // description?: string является необязательным и опущен
};

const response: CreateTicketResponse1 = await createTicket(tenantId, userId, ticketBody);
// Пример использования необязательного поля из ответа
// console.log(response.ticket?.id);
[inline-code-end]

---