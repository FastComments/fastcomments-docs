## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|------|-------------|
| tenantId | string | 예 |  |
| userId | string | 예 |  |
| id | string | 예 |  |
| changeTicketStateBody | ChangeTicketStateBody | 예 |  |

## 응답

반환: [`ChangeTicketState200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ChangeTicketState200Response.ts)

## 예제

[inline-code-attrs-start title = 'changeTicketState 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_82f9b3';
const userId: string = 'user_9472';
const id: string = 'ticket_550e8400-e29b-41d4-a716-446655440000';
const changeTicketStateBody: ChangeTicketStateBody = {
  state: 'resolved',
  comment: 'Confirmed fix deployed to production; closing ticket.',
  notifySubscribers: true
};
const result: ChangeTicketState200Response = await changeTicketState(tenantId, userId, id, changeTicketStateBody);
[inline-code-end]

---