
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
const tenantId: string = 'tenant_8a1f4';
const maybeUrlId: string | undefined = 'articles/2025/product-launch';
const urlId: string = maybeUrlId!;
const userIdWS: string = 'ws_user_4521';
const startTime: number = Date.now() - 1000 * 60 * 60 * 24; // 24 hours ago
const endTime: number = Date.now();
const result: GetEventLog200Response = await getEventLog(tenantId, urlId, userIdWS, startTime, endTime);
[inline-code-end]
