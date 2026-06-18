---
## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| userId | string | Так |  |
| id | string | Так |  |
| changeTicketStateBody | ChangeTicketStateBody | Так |  |

## Відповідь

Повертає: [`ChangeTicketState200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ChangeTicketState200Response.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад changeTicketState'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8a7d3f4b';
const userId: string = 'user_5d1a9b2c';
const id: string = 'ticket_1024';
const changeTicketStateBody: ChangeTicketStateBody = {
  state: 'closed',
  notifyParticipants: true, // продемонстровано необов'язковий параметр
  comment: 'Resolved by support — follow-up not required.'
};
const result: ChangeTicketState200Response = await changeTicketState(tenantId, userId, id, changeTicketStateBody);
[inline-code-end]

---