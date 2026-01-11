## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| voteId | string | Yes |  |
| urlId | string | Yes |  |
| broadcastId | string | Yes |  |
| editKey | string | No |  |
| sso | string | No |  |

## Response

Returns: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteCommentVote200Response.ts)

## Example

[inline-code-attrs-start title = 'deleteCommentVote Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async function run(): Promise<void> {
  const tenantId: string = "tenant_9f3d2b1a";
  const commentId: string = "comment_72a9f4e1";
  const voteId: string = "vote_8d1a3c5b";
  const urlId: string = "url_4b6d9e2f";
  const broadcastId: string = "broadcast_2025-11-22_01";
  const editKey: string | undefined = "editkey_6a1b2c3d";
  const sso: string | undefined = "sso_jwt_token_example";
  const result: DeleteCommentVote200Response = await deleteCommentVote(tenantId, commentId, voteId, urlId, broadcastId, editKey, sso);
  console.log(result);
})();
[inline-code-end]
