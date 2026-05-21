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
const tenantId: string = 'enterprise-acme-123';
const id: string = 'f47ac10b-58cc-4372-a567-0e02b2c3d479';
const deleteComments: boolean = true;
const commentDeleteMode: string = 'hard';
const result: DeleteSSOUserAPIResponse = await deleteSSOUser(tenantId, id, deleteComments, commentDeleteMode);
[inline-code-end]
