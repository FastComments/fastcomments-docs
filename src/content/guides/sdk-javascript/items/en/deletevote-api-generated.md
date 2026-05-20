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
async function demo(): Promise<void> {
  const tenantId: string = "acme-corp";
  const id: string = "comment-4829";
  const editKey: string = "edit_9f3a2b7c";
  const resultWithKey: DeleteCommentVote200Response = await deleteVote(tenantId, id, editKey);
  const resultWithoutKey: DeleteCommentVote200Response = await deleteVote(tenantId, id);
  console.log(resultWithKey, resultWithoutKey);
}
demo();
[inline-code-end]
