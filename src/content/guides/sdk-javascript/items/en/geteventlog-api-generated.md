
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
const tenantId: string = "tenant_acme_01";
const urlId: string = "url_742";
const userIdWS: string = "ws_user_97";
const startTime: number = Date.now() - 24 * 60 * 60 * 1000;
const optionalEndTime: number | undefined = undefined;
const endTime: number = optionalEndTime ?? Date.now();
const result: GetEventLog200Response = await getEventLog(tenantId, urlId, userIdWS, startTime, endTime);
[inline-code-end]
