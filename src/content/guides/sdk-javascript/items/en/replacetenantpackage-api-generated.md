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
(async () => {
  const tenantId: string = 'tenant_8a1f';
  const id: string = 'pkg_bronze_2026';
  const replaceTenantPackageBody: ReplaceTenantPackageBody = {
    name: 'Bronze Plan',
    description: 'Entry tier with basic moderation and analytics',
    apiStatus: { status: 'active' },
    customConfigParameters: { maxCommentsPerUser: 100 }
  } as ReplaceTenantPackageBody;
  const result: FlagCommentPublic200Response = await replaceTenantPackage(tenantId, id, replaceTenantPackageBody);
  console.log(result);
})();
[inline-code-end]
