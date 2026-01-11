## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| limit | number | No |  |
| skip | number | No |  |
| order | SORTDIR | No |  |
| after | number | No |  |
| before | number | No |  |

## Response

Returns: [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetAuditLogs200Response.ts)

## Example

[inline-code-attrs-start title = 'getAuditLogs Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f7b2c';
const limit: number = 100;
const skip: number = 0;
const order: SORTDIR = 'desc' as unknown as SORTDIR;
const after: number = Math.floor(Date.now() / 1000) - 7 * 24 * 60 * 60;
const result: GetAuditLogs200Response = await getAuditLogs(tenantId, limit, skip, order, after);
[inline-code-end]
