## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateTenantPackageBody | UpdateTenantPackageBody | Yes |  |

## Response

Returns: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Example

[inline-code-attrs-start title = 'updateTenantPackage Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_4b7c9a2f";
const id: string = "pkg_91f2d3b8";
const updateTenantPackageBody: UpdateTenantPackageBody = {
  planId: "business_annual",
  seats: 50,
  autoRenew: true,
  couponCode: "WELCOME2025" // optional parameter demonstrated
};
const result: APIEmptyResponse = await updateTenantPackage(tenantId, id, updateTenantPackageBody);
[inline-code-end]
