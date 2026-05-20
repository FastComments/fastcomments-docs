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
const tenantId: string = 'acme-corp-tenant-42';
const usersFirstPage: GetTenantUsers200Response = await getTenantUsers(tenantId);
const usersSecondPage: GetTenantUsers200Response = await getTenantUsers(tenantId, 50);
[inline-code-end]
