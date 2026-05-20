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
const tenantId: string = 'tenant-9b72';
const id: string = 'pkg-2026-01';
const body: ReplaceTenantPackageBody = {
  name: 'Core Moderation Package',
  version: '2026.05.01',
  apiStatus: { status: 'active', updatedAt: '2026-05-01T12:00:00Z' } as APIStatus,
  customConfig: { maxCommentsPerUser: 100, enableSpamFilter: true } as CustomConfigParameters
} as ReplaceTenantPackageBody;
const response: FlagCommentPublic200Response = await replaceTenantPackage(tenantId, id, body);
[inline-code-end]
