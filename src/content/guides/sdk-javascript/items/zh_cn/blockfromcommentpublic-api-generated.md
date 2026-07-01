## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | 是 |  |
| sso | string | 否 |  |

## 响应

返回: [`BlockFromCommentPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BlockFromCommentPublicResponse.ts)

## 示例

[inline-code-attrs-start title = 'blockFromCommentPublic 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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