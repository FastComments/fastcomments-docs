
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
const tenantId: string = "tenant-84b2f1";
const urlId: string = "article-6721";
const userIdWS: string = "ws-conn-9a3c";
const startTime: number = Date.now() - 7 * 24 * 60 * 60 * 1000; // 7 days ago
const endTimeOptional: number | undefined = undefined; // optional timeframe end
const endTime: number = endTimeOptional ?? Date.now();
const eventLog: GetEventLog200Response = await getGlobalEventLog(tenantId, urlId, userIdWS, startTime, endTime);
[inline-code-end]
