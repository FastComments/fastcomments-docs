## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| replaceTenantUserBody | ReplaceTenantUserBody | Yes |  |
| updateComments | string | No |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'replaceTenantUser Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_a8b3f0e2';
const id: string = 'user_fb2a1e9d';
const replaceTenantUserBody: ReplaceTenantUserBody = {
  externalId: 'ext-98765',
  email: 'jane.doe@example.com',
  displayName: 'Jane Doe',
  role: 'member'
} as ReplaceTenantUserBody;
const updateComments: string = 'true';
const result: FlagCommentPublic200Response = await replaceTenantUser(tenantId, id, replaceTenantUserBody, updateComments);
[inline-code-end]
