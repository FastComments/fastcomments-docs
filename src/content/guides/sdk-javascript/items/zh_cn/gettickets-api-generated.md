## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| userId | string | 否 |  |
| state | number | 否 |  |
| skip | number | 否 |  |
| limit | number | 否 |  |

## 响应

返回: [`GetTickets200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTickets200Response.ts)

## 示例

[inline-code-attrs-start title = 'getTickets 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_92f3b4c1";
const userId: string = "user_742a9f3e";
const state: number = 1;
const skip: number = 0;
const limit: number = 25;
const ticketsFull: GetTickets200Response = await getTickets(tenantId, userId, state, skip, limit);
const ticketsMinimal: GetTickets200Response = await getTickets("tenant_92f3b4c1");
[inline-code-end]

---