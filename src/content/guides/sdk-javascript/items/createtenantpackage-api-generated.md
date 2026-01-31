## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createTenantPackageBody | CreateTenantPackageBody | Yes |  |

## Response

Returns: [`CreateTenantPackage200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantPackage200Response.ts)

## Example

[inline-code-attrs-start title = 'createTenantPackage Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_89c2f1";
const createTenantPackageBody: CreateTenantPackageBody = {
  name: "Premium Moderation",
  planId: "premium-2026",
  seats: 50,
  ssoSecurityLevel: "strict" as SSOSecurityLevel,
  customConfigParameters: { theme: "dark", avatarMaxSizeKB: 512 } as CustomConfigParameters,
  commentRenderingMode: "trusted" as CommentHTMLRenderingMode
} as CreateTenantPackageBody;
const result: CreateTenantPackage200Response = await createTenantPackage(tenantId, createTenantPackageBody);
[inline-code-end]
