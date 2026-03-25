## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| userId | string | Не |  |
| state | number | Не |  |
| skip | number | Не |  |
| limit | number | Не |  |

## Отговор

Връща: [`GetTickets200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTickets200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getTickets'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_92f3b4c1";
const userId: string = "user_742a9f3e";
const state: number = 1;
const skip: number = 0;
const limit: number = 25;
const ticketsFull: GetTickets200Response = await getTickets(tenantId, userId, state, skip, limit);
const ticketsMinimal: GetTickets200Response = await getTickets("tenant_92f3b4c1");
[inline-code-end]

---