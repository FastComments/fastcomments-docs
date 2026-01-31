## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| skip | number | No |  |

## Response

Returns: [`GetTenantPackages200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackages200Response.ts)

## Example

[inline-code-attrs-start title = 'getTenantPackages Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "8a3f5b2e-1c9d-4f2b-b123-9f0d6e4a5c12";
const skip: number = 20;
const packagesWithSkip: GetTenantPackages200Response = await getTenantPackages(tenantId, skip);
const packagesWithoutSkip: GetTenantPackages200Response = await getTenantPackages(tenantId);
[inline-code-end]
