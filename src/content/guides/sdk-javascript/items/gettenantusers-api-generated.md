## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| skip | number | No |  |

## Response

Returns: [`GetTenantUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUsers200Response.ts)

## Example

[inline-code-attrs-start title = 'getTenantUsers Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "f7a9b1d2-3c4e-5f6a-7890-abc123def456";
const skip: number = 50;
const responseWithSkip: GetTenantUsers200Response = await getTenantUsers(tenantId, skip);
const responseNoSkip: GetTenantUsers200Response = await getTenantUsers(tenantId);
[inline-code-end]
