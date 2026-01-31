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
const tenantId: string = "tenant_6a1f9b2";
const limit: number = 50;
const skip: number = 0;
const order: SORTDIR = SortDirections.DESC;
const after: number = Date.now() - 30 * 24 * 60 * 60 * 1000;
const before: number = Date.now();

const response: GetAuditLogs200Response = await getAuditLogs(tenantId, limit, skip, order, after, before);
[inline-code-end]
