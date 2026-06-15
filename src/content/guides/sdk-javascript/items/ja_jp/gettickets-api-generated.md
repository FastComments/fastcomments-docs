---
## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | No |  |
| state | number | No |  |
| skip | number | No |  |
| limit | number | No |  |

## レスポンス

戻り値: [`GetTickets200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTickets200Response.ts)

## 例

[inline-code-attrs-start title = 'getTickets の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_01';
const userId: string = 'user_87b3';
const state: number = 2;
const skip: number = 0;
const limit: number = 50;

const tickets: GetTickets200Response = await getTickets(tenantId, userId, state, skip, limit);
[inline-code-end]

---