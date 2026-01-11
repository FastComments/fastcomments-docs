## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| skip | number | No |  |

## Response

Returns: [`GetSSOUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSSOUsers200Response.ts)

## Example

[inline-code-attrs-start title = 'getSSOUsers Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f8b3c4d-01';
const skip: number = 50;
const usersFirstPage: GetSSOUsers200Response = await getSSOUsers(tenantId);
const usersSecondPage: GetSSOUsers200Response = await getSSOUsers(tenantId, skip);
[inline-code-end]
