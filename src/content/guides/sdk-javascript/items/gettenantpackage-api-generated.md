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
const tenantId: string = 'tenant_acme_corp_01';
const packageId: string = 'package_enterprise_2026';
const result: GetTenantPackage200Response = await getTenantPackage(tenantId, packageId);
const tenantPackage: TenantPackage | undefined = result?.tenantPackage;
const customConfig: CustomConfigParameters | undefined = tenantPackage?.customConfigParameters;
[inline-code-end]
