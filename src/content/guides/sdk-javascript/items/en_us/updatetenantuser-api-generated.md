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
const tenantId: string = 'tenant_84f3b2';
const id: string = 'user_7a9d1c';
const updateComments: string = 'Promoted to moderator and updated contact email';
const updateTenantUserBody: UpdateTenantUserBody = {
  email: 'jane.doe+mod@example.com',
  displayName: 'Jane D.',
  roles: ['moderator'],
  isBanned: false,
  metadata: { department: 'community' }
};
const result: FlagCommentPublic200Response = await updateTenantUser(tenantId, id, updateTenantUserBody, updateComments);
[inline-code-end]