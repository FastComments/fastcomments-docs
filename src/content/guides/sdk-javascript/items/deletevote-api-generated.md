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
(async () => {
  const tenantId: string = 'tenant_8f3b2c9d';
  const id: string = 'comment_60a3f1b2';
  const editKey: string = 'ed_kJ3n9PxV'; // optional edit key when modifying a previously created vote
  const resultWithEditKey: DeleteCommentVote200Response = await deleteVote(tenantId, id, editKey);
  const resultWithoutEditKey: DeleteCommentVote200Response = await deleteVote(tenantId, id);
  console.log(resultWithEditKey, resultWithoutEditKey);
})();
[inline-code-end]
