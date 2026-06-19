## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| userId | string | 是 |  |
| id | string | 是 |  |
| changeTicketStateBody | ChangeTicketStateBody | 是 |  |

## 回應

回傳： [`ChangeTicketStateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ChangeTicketStateResponse.ts)

## 範例

[inline-code-attrs-start title = 'changeTicketState 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_01';
const userId: string = 'moderator_421';
const id: string = 'ticket_8421';
const changeTicketStateBody: ChangeTicketStateBody = { state: 'closed', reason: 'Resolved after user follow-up', notifyUsers: true } as ChangeTicketStateBody;
const result: ChangeTicketStateResponse = await changeTicketState(tenantId, userId, id, changeTicketStateBody);
[inline-code-end]

---