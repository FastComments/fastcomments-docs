## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createTenantPackageBody | CreateTenantPackageBody | Yes |  |

## Response

Returns: [`CreateTenantPackage200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantPackage200Response.ts)

## Example

[inline-code-attrs-start title = 'createTenantPackage 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme-corp_001";
const createTenantPackageBody: CreateTenantPackageBody = {
  name: "Acme Standard Package",
  description: "Default package for Acme Corp comments with moderation and SSO enabled",
  enabled: true,
  maxCommentsPerThread: 500,
  voteStyle: "thumbs",
  gifRating: "PG-13",
  tosConfig: { enabled: true, url: "https://acme.example.com/terms" } // 示範可選參數
};
const result: CreateTenantPackage200Response = await createTenantPackage(tenantId, createTenantPackageBody);
[inline-code-end]