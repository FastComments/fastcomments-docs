## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateTenantPackageBody | UpdateTenantPackageBody | Yes |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'updateTenantPackage Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_3b7f9d-prod";
const id: string = "pkg_enterprise_2026";
const updateTenantPackageBody: UpdateTenantPackageBody = {
  name: "Enterprise Plus",
  isActive: true,
  // optional fields omitted intentionally (e.g., description, limits)
} as UpdateTenantPackageBody;
const result: FlagCommentPublic200Response = await updateTenantPackage(tenantId, id, updateTenantPackageBody);
[inline-code-end]
