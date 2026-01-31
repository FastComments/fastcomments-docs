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
const tenantId: string = 'a3f9c2d4-7b6e-4f11-9d88-2c6b5a7e8f90';
const packageId: string = 'premium_monthly_2026';
const updateTenantPackageBody: UpdateTenantPackageBody = {
  displayName: 'Premium Moderation Plan',
  planType: 'enterprise',
  retentionDays: 365,
  moderationEnabled: true,
  supportEmail: 'ops+moderation@acme-corp.com'
};
const result: FlagCommentPublic200Response = await updateTenantPackage(tenantId, packageId, updateTenantPackageBody);
[inline-code-end]
