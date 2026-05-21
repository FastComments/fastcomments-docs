## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| meta | string | No |  |
| skip | number | No |  |

## Response

Returns: [`GetTenants200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenants200Response.ts)

## Example

[inline-code-attrs-start title = 'getTenants Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'org_9a12f4';
const meta: string = 'include=domains,billing';
const skip: number = 20;
const tenantsResponse: GetTenants200Response = await getTenants(tenantId, meta, skip);
[inline-code-end]
