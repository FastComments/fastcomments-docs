## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| userId | string | 是 |  |
| id | string | 是 |  |
| changeTicketStateBody | ChangeTicketStateBody | 是 |  |

## 响应

返回: [`ChangeTicketState200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ChangeTicketState200Response.ts)

## 示例

[inline-code-attrs-start title = 'changeTicketState 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8a7d3f4b';
const userId: string = 'user_5d1a9b2c';
const id: string = 'ticket_1024';
const changeTicketStateBody: ChangeTicketStateBody = {
  state: 'closed',
  notifyParticipants: true, // 演示可选参数
  comment: 'Resolved by support — follow-up not required.'
};
const result: ChangeTicketState200Response = await changeTicketState(tenantId, userId, id, changeTicketStateBody);
[inline-code-end]

---