## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| userId | string | No |  |

## Response

Returns: [`GetTicket200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTicket200Response.ts)

## Example

[inline-code-attrs-start title = 'getTicket Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_acme_01';
  const id: string = 'TCK-20260520-001';
  const ticket: GetTicket200Response = await getTicket(tenantId, id);
  const userId: string = 'user_98765';
  const userTicket: GetTicket200Response = await getTicket(tenantId, 'TCK-20260520-002', userId);
  console.log(ticket, userTicket);
})();
[inline-code-end]
