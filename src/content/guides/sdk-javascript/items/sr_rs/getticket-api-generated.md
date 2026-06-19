## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |
| userId | string | Не |  |

## Одговор

Враћа: [`GetTicketResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTicketResponse.ts)

## Пример

[inline-code-attrs-start title = 'getTicket Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const ticketResponse: GetTicketResponse = await getTicket("fc_tenant_1a2b3c", "TK-20260619-0042", "user_2481");
const ticketResponseNoUser: GetTicketResponse = await getTicket("fc_tenant_1a2b3c", "TK-20260619-0043");
[inline-code-end]