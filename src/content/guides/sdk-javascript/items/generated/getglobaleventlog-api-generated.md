
req
tenantId
urlId
userIdWS

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| userIdWS | string | Yes |  |
| startTime | number | Yes |  |
| endTime | number | Yes |  |

## Response

Returns: [`GetEventLog200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEventLog200Response.ts)

## Example

[inline-code-attrs-start title = 'getGlobalEventLog Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme_media_tenant_001';
const urlId: string = 'article-2025-11-22-fiber-upgrade';
const userIdWS: string = 'ws_user_78321';
const startTime: number = Date.now() - 1000 * 60 * 60 * 24; // 24 hours ago
const endTime: number = Date.now();
// Optional parameters example (not required by getGlobalEventLog)
const options: { includeMetadata?: boolean; maxEntries?: number } = { includeMetadata: true };
const result: GetEventLog200Response = await getGlobalEventLog(tenantId, urlId, userIdWS, startTime, endTime);
console.log(result);
[inline-code-end]
