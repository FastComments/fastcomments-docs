## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| userId | string | はい |  |
| id | string | はい |  |
| changeTicketStateBody | ChangeTicketStateBody | はい |  |

## レスポンス

戻り値: [`ChangeTicketState200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ChangeTicketState200Response.ts)

## 例

[inline-code-attrs-start title = 'changeTicketState の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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