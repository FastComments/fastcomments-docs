## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Yes |  |
| sso | string | No |  |

## 响应

Returns: [`UnBlockCommentPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnBlockCommentPublicResponse.ts)

## 示例

[inline-code-attrs-start title = 'unBlockCommentPublic 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant";
const commentId: string = "cmt-20230915-001";
const unblockParams: PublicBlockFromCommentParams = {
  reason: "User appealed and was cleared",
  unblockExpiresAt: new Date(Date.now() + 24 * 60 * 60 * 1000).toISOString()
};
const ssoToken: string = "sso-3d9f8a7b";

const result: UnBlockCommentPublicResponse = await unBlockCommentPublic(
  tenantId,
  commentId,
  unblockParams,
  ssoToken
);
[inline-code-end]