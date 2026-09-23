## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | No |  |
| state | number | No |  |
| skip | number | No |  |
| limit | number | No |  |

## 响应

Returns: [`GetTicketsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTicketsResponse.ts)

## 示例

[inline-code-attrs-start title = 'getTickets 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const ticketsSimple: GetTicketsResponse = await getTickets(tenantId);

  const userId: string = "user_9876";
  const state: number = 1; // 例如，打开
  const skip: number = 0;
  const limit: number = 20;
  const ticketsFull: GetTicketsResponse = await getTickets(
    tenantId,
    userId,
    state,
    skip,
    limit
  );
})();
[inline-code-end]