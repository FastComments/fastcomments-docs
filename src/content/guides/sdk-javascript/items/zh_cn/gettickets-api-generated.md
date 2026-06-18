## 参数

| Name | Type | Required | Description |
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
const tenantId: string = 'tenant_acme_01';
const userId: string = 'user_87b3';
const state: number = 2;
const skip: number = 0;
const limit: number = 50;

const tickets: GetTickets200Response = await getTickets(tenantId, userId, state, skip, limit);
[inline-code-end]

---