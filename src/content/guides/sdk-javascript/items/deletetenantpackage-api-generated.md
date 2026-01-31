## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'deleteTenantPackage Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f3c2b9a';
const packageId: string = 'pkg_prod_cleanup_20260109';
const dryRun: boolean | undefined = true; // optional flag example (not required by function)
const response: FlagCommentPublic200Response = await deleteTenantPackage(tenantId, packageId);
[inline-code-end]
