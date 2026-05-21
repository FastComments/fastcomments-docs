## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`GetTenantPackage200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackage200Response.ts)

## Example

[inline-code-attrs-start title = 'getTenantPackage Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'org-6412';
const id: string = 'pkg-7f3b1c4a-9a2d-4e3b-b8c2-1a2b3c4d5e6f';
const includePreview: boolean | undefined = undefined; // optional parameter example
const packageResponse: GetTenantPackage200Response = await getTenantPackage(tenantId, id);
[inline-code-end]
