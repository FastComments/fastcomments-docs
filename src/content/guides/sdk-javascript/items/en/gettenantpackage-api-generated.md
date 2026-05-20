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
(async () => {
  const tenantId: string = 'tenant_8f3b2c9e-01';
  const packageId: string = 'pkg-47a9d2b1';
  const includeRelated: boolean | undefined = true;
  const tenantPackageResponse: GetTenantPackage200Response = await getTenantPackage(tenantId, packageId);
  console.log(tenantPackageResponse);
})();
[inline-code-end]
