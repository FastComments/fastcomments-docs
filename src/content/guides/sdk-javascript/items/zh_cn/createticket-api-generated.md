## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| userId | string | 是 |  |
| createTicketBody | CreateTicketBody | 是 |  |

## 响应

返回：[`CreateTicket200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTicket200Response.ts)

## 示例

[inline-code-attrs-start title = 'createTicket 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-company-001';
const userId: string = 'u_78f4b2';
const createTicketBody: CreateTicketBody = {
  title: 'Unable to access project dashboard',
  description: 'Receiving 403 when accessing /dashboard for project X',
  priority: 'high',
  tags: ['dashboard', 'access'] // 可选字段示例
};
const result: CreateTicket200Response = await createTicket(tenantId, userId, createTicketBody);
[inline-code-end]

---