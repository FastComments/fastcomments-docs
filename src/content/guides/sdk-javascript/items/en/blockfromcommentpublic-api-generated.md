## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Yes |  |
| sso | string | No |  |

## Response

Returns: [`BlockFromCommentPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BlockFromCommentPublicResponse.ts)

## Example

[inline-code-attrs-start title = 'blockFromCommentPublic Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demo() {
  const tenantId: string = "tenant_9f8b7c";
  const commentId: string = "cmt_1234567890";
  const blockParams: PublicBlockFromCommentParams = {
    reason: "spam",
    durationHours: 24,
  };
  const ssoToken: string = "sso_ABCDEF123456";

  const responseWithSso: BlockFromCommentPublicResponse = await blockFromCommentPublic(
    tenantId,
    commentId,
    blockParams,
    ssoToken
  );

  const responseWithoutSso: BlockFromCommentPublicResponse = await blockFromCommentPublic(
    tenantId,
    commentId,
    blockParams
  );

  console.log(responseWithSso, responseWithoutSso);
}
demo();
[inline-code-end]
