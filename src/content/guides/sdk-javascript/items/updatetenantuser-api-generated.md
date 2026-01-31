## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| updateTenantUserBody | UpdateTenantUserBody | Yes |  |
| updateComments | string | No |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'updateTenantUser Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_84f9c2';
const id: string = 'user_7b8c9a';
const updateTenantUserBody: UpdateTenantUserBody = {
  email: 'jane.doe@acmepublishing.com',
  displayName: 'Jane Doe',
  roles: ['moderator', 'editor'],
  isActive: true,
  customConfig: { moderationQueueEnabled: true }
};
const updateComments: string = 'Update display name and roles; apply to existing comments';
const result: FlagCommentPublic200Response = await updateTenantUser(tenantId, id, updateTenantUserBody, updateComments);
[inline-code-end]
