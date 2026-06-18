## パラメータ

| Name | Type | Required | Description |
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
const tenantId: string = 'tenant_8a7d3f4b';
const userId: string = 'user_5d1a9b2c';
const id: string = 'ticket_1024';
const changeTicketStateBody: ChangeTicketStateBody = {
  state: 'closed',
  notifyParticipants: true, // 任意のパラメータの例
  comment: 'Resolved by support — follow-up not required.'
};
const result: ChangeTicketState200Response = await changeTicketState(tenantId, userId, id, changeTicketStateBody);
[inline-code-end]

---