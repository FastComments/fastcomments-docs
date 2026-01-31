## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| deleteComments | boolean | No |  |
| commentDeleteMode | string | No |  |

## Response

Returns: [`DeleteSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteSSOUserAPIResponse.ts)

## Example

[inline-code-attrs-start title = 'deleteSSOUser Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_6b2f1c9a';
const userId: string = 'sso_user_84b3c2';
const deleteComments: boolean = true;
const commentDeleteMode: string = 'permanent';

const responseWithDeletion: DeleteSSOUserAPIResponse = await deleteSSOUser(tenantId, userId, deleteComments, commentDeleteMode);
const responseWithoutDeletion: DeleteSSOUserAPIResponse = await deleteSSOUser(tenantId, userId);
[inline-code-end]
