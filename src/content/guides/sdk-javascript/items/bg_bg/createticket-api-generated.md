## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| userId | string | Да |  |
| createTicketBody | CreateTicketBody | Да |  |

## Отговор

Връща: [`CreateTicket200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTicket200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример за createTicket'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_corp';
const userId: string = 'moderator_jane';
const createTicketBody: CreateTicketBody = {
  subject: 'Mass spam reports on article 789',
  description: 'Multiple identical spam comments posted under article 789. Needs moderation and bulk removal.',
  priority: 'high',
  contactEmail: 'jane@acme-corp.com',
  metadata: { articleId: '789', reportedCount: 12 } // пример за незадължителни метаданни
};
const ticket: CreateTicket200Response = await createTicket(tenantId, userId, createTicketBody);
[inline-code-end]