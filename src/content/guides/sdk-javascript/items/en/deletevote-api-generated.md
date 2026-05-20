## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| editKey | string | No |  |

## Response

Returns: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteCommentVote200Response.ts)

## Example

[inline-code-attrs-start title = 'deleteVote Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8b3f2a9c';
const id: string = 'comment-4721';
const editKey: string = 'edk_4f2a9b7c';
const responseWithEditKey: DeleteCommentVote200Response = await deleteVote(tenantId, id, editKey);
const responseWithoutEditKey: DeleteCommentVote200Response = await deleteVote(tenantId, id);
[inline-code-end]
