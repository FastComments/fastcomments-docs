
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
const tenantId: string = "tenant_4f7b2c9d";
const urlId: string = "url_805c3a1b";
const userIdWS: string = "ws_user_102";
const startTime: number = Date.now() - 7 * 24 * 60 * 60 * 1000; // one week ago
const endTime: number = Date.now();
const pageSize: number | undefined = 250; // optional parameter example (not required by getEventLog)
const result: GetEventLog200Response = await getEventLog(tenantId, urlId, userIdWS, startTime, endTime);
[inline-code-end]
