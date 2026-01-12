
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

[inline-code-attrs-start title = 'getEventLog Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f9c3b';
const urlId: string = 'article-3f2a1c';
const userIdWS: string = 'ws-user-4521';
const startTime: number = Date.now() - 86_400_000; // 24 hours ago (ms)
const endTime: number = Date.now();
const endTimeOverride: number | undefined = undefined; // optional override
const endTimeToUse: number = endTimeOverride ?? endTime;
const response: GetEventLog200Response = await getEventLog(tenantId, urlId, userIdWS, startTime, endTimeToUse);
[inline-code-end]
