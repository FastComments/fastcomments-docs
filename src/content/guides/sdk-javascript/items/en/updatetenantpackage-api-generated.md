## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateTenantPackageBody | UpdateTenantPackageBody | Yes |  |

## Response

Returns: [`UpdateTenantPackageResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateTenantPackageResponse.ts)

## Example

[inline-code-attrs-start title = 'updateTenantPackage Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant-9876";
const packageId: string = "pkg-2023";

const customConfig: CustomConfigParameters = {
  enableSpamFilter: true,
  spamRatingThreshold: 4,
};

const updateBody: UpdateTenantPackageBody = {
  displayName: "Enterprise Pro",
  customConfig,
};

const response: UpdateTenantPackageResponse = await updateTenantPackage(
  tenantId,
  packageId,
  updateBody
);
[inline-code-end]
