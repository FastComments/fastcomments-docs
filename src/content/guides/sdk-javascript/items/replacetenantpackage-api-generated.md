## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| replaceTenantPackageBody | ReplaceTenantPackageBody | Yes |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'replaceTenantPackage Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_4c8b9";
const id: string = "pkg_pro_2026";
const replaceTenantPackageBody: ReplaceTenantPackageBody = {
  displayName: "Pro Moderation Pack",
  status: "active" as APIStatus,
  // optional configuration provided; other optional fields omitted
  customConfig: { maxCommentLength: 1000, enableProfanityFilter: true } as CustomConfigParameters
};
const result: FlagCommentPublic200Response = await replaceTenantPackage(tenantId, id, replaceTenantPackageBody);
[inline-code-end]
