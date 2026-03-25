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
const tenantId: string = 'acme-corp-tenant-01';
const ticketId: string = 'tkt-20260325-42';
const userId: string = 'user-8452';

const ticketResponseWithUser: GetTicket200Response = await getTicket(tenantId, ticketId, userId);
const ticketResponseWithoutUser: GetTicket200Response = await getTicket(tenantId, ticketId);
[inline-code-end]

---