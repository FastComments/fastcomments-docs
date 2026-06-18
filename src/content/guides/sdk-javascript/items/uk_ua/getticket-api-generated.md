## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Так |  |
| userId | string | Ні |  |

## Відповідь

Повертає: [`GetTicket200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTicket200Response.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getTicket'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp';
const ticketId: string = 'TCKT-20250615-42';
const userId: string = 'user_84b2';

const ticketWithUser: GetTicket200Response = await getTicket(tenantId, ticketId, userId);
const ticketWithoutUser: GetTicket200Response = await getTicket(tenantId, ticketId);

console.log(ticketWithUser.id, ticketWithoutUser.id);
[inline-code-end]

---