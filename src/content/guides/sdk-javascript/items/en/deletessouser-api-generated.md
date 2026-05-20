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
const tenantId: string = 'acme-enterprises-01';
const id: string = 'sso_8a3d4f2b';
const deleteComments: boolean = true;
const commentDeleteMode: string = 'permanent';
const result: DeleteSSOUserAPIResponse = await deleteSSOUser(tenantId, id, deleteComments, commentDeleteMode);
[inline-code-end]
