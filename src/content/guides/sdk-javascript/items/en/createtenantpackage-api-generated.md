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
const tenantId: string = "tenant_acme_corp_001";
const createTenantPackageBody: CreateTenantPackageBody = {
  packageName: "Acme Moderation Pack",
  version: "1.2.0",
  description: "Moderation settings tailored for Acme forums",
  allowGuestPosting: false,
  customConfig: {
    profanityLevel: "medium",
    imageMaxMb: 8
  }
} as CreateTenantPackageBody;
const result: CreateTenantPackage200Response = await createTenantPackage(tenantId, createTenantPackageBody);
[inline-code-end]
