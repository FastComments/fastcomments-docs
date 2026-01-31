
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
const tenantId: string = 'tenant_9b2f4c1a';
const urlId: string = 'https://news.example.com/articles/2026/01/09/new-feature';
const userIdWS: string = 'ws_user_7421';
const startTime: number = Date.now() - 7 * 24 * 60 * 60 * 1000; // one week ago
const endTime: number = Date.now();
const result: GetEventLog200Response = await getGlobalEventLog(tenantId, urlId, userIdWS, startTime, endTime);
[inline-code-end]
