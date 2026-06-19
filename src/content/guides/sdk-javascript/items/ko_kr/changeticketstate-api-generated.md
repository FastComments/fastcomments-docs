## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| userId | string | 예 |  |
| id | string | 예 |  |
| changeTicketStateBody | ChangeTicketStateBody | 예 |  |

## 응답

반환: [`ChangeTicketStateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ChangeTicketStateResponse.ts)

## 예제

[inline-code-attrs-start title = 'changeTicketState 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_01';
const userId: string = 'moderator_421';
const id: string = 'ticket_8421';
const changeTicketStateBody: ChangeTicketStateBody = { state: 'closed', reason: 'Resolved after user follow-up', notifyUsers: true } as ChangeTicketStateBody;
const result: ChangeTicketStateResponse = await changeTicketState(tenantId, userId, id, changeTicketStateBody);
[inline-code-end]

---