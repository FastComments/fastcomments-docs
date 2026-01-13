## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Yes |  |
| sso | string | No |  |

## Response

Returns: [`UnBlockCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnBlockCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'unBlockCommentPublic Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoUnblock(): Promise<void> {
  const tenantId: string = 'tenant_12345';
  const commentId: string = 'cmt_98765';
  const publicBlockFromCommentParams: PublicBlockFromCommentParams = {
    unblockReason: 'User appeal accepted after review',
    moderatorId: 'mod_67890',
    liftedAt: new Date().toISOString()
  };
  const sso: string = 'sso_tok_prod_01ab2c3d';
  const result: UnBlockCommentPublic200Response = await unBlockCommentPublic(tenantId, commentId, publicBlockFromCommentParams, sso);
  console.log(result);
}
demoUnblock();
[inline-code-end]
