
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
const tenantId: string = 'tenant_7a3f2b';
const urlId: string = 'site-homepage';
const optionalUrlId: string | undefined = undefined;
const userIdWS: string = 'ws-user-9843';
const startTime: number = Date.now() - 24 * 60 * 60 * 1000;
const endTime: number = Date.now();
const eventLog: GetEventLog200Response = await getGlobalEventLog(tenantId, optionalUrlId ?? urlId, userIdWS, startTime, endTime);
[inline-code-end]
