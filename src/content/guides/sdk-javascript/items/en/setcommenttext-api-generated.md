## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| broadcastId | string | Yes |  |
| commentTextUpdateRequest | CommentTextUpdateRequest | Yes |  |
| editKey | string | No |  |
| sso | string | No |  |

## Response

Returns: [`SetCommentText200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SetCommentText200Response.ts)

## Example

[inline-code-attrs-start title = 'setCommentText Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_7a9f3";
  const commentId: string = "comment_4821";
  const broadcastId: string = "broadcast_2026_05_20";
  const commentTextUpdateRequest: CommentTextUpdateRequest = { text: "Thanks for the update — I added more detail to clarify next steps." };
  const editKey: string | undefined = "editkey_6b2c";
  const sso: string | undefined = "sso_token_eyJhbGci";
  const result: SetCommentText200Response = await setCommentText(tenantId, commentId, broadcastId, commentTextUpdateRequest, editKey, sso);
  console.log(result);
})();
[inline-code-end]
