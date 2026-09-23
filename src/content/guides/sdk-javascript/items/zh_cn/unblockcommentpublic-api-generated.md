## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | Yes |  |
| sso | string | No |  |

## 响应

返回: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnblockSuccess.ts)

## 示例

[inline-code-attrs-start title = 'unBlockCommentPublic 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "c9f1e2a4-5b6d-4f8a-9c2e-123456789abc";
const commentId: string = "d3b2c1a9-8f7e-4d6b-9a0b-987654321def";

const unblockParams: PublicBlockFromCommentParams = {
    reason: "User request resolved",
    moderatorId: "mod-456"
};

const ssoToken: string = "sso-token-789xyz";

const unblockResult: UnblockSuccess = await unBlockCommentPublic(tenantId, commentId, unblockParams, ssoToken);
const unblockResultNoSso: UnblockSuccess = await unBlockCommentPublic(tenantId, commentId, unblockParams);
[inline-code-end]