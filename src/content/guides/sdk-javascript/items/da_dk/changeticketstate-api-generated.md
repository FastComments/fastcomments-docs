## Parametre

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| userId | string | Ja |  |
| id | string | Ja |  |
| changeTicketStateBody | ChangeTicketStateBody | Ja |  |

## Svar

Returnerer: [`ChangeTicketStateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ChangeTicketStateResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'changeTicketState Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_01';
const userId: string = 'moderator_421';
const id: string = 'ticket_8421';
const changeTicketStateBody: ChangeTicketStateBody = { state: 'closed', reason: 'Resolved after user follow-up', notifyUsers: true } as ChangeTicketStateBody;
const result: ChangeTicketStateResponse = await changeTicketState(tenantId, userId, id, changeTicketStateBody);
[inline-code-end]

---