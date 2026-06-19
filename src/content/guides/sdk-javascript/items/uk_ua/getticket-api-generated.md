## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Так |  |
| userId | string | Ні |  |

## Відповідь

Повертає: [`GetTicketResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTicketResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getTicket'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const ticketResponse: GetTicketResponse = await getTicket("fc_tenant_1a2b3c", "TK-20260619-0042", "user_2481");
const ticketResponseNoUser: GetTicketResponse = await getTicket("fc_tenant_1a2b3c", "TK-20260619-0043");
[inline-code-end]

---