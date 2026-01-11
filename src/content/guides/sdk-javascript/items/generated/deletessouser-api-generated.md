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
const tenantId: string = 'tenant_84b2f1e';
const id: string = 'ssouser_5a9c3d2';
const deleteComments: boolean = true;
const commentDeleteMode: string = 'permanent';
const response: DeleteSSOUserAPIResponse = await deleteSSOUser(tenantId, id, deleteComments, commentDeleteMode);
[inline-code-end]
