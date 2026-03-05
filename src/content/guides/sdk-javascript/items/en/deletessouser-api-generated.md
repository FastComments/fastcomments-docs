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
const tenantId: string = 'tenant_8a3f4c2b-6d1a';
const id: string = 'sso_user_3f9a7b1d-22c4';
const deleteComments: boolean = true;
const commentDeleteMode: string = 'permanent';
const responseWithDeletion: DeleteSSOUserAPIResponse = await deleteSSOUser(tenantId, id, deleteComments, commentDeleteMode);
const responseWithoutDeletion: DeleteSSOUserAPIResponse = await deleteSSOUser(tenantId, id);
[inline-code-end]
