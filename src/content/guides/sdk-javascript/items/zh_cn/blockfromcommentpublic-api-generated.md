## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| publicBlockFromCommentParams | PublicBlockFromCommentParams | 是 |  |
| sso | string | 否 |  |

## 响应

返回：[`BlockSuccess`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BlockSuccess.ts)

## 示例

[inline-code-attrs-start title = 'blockFromCommentPublic 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_52b9f3a1";
const commentId: string = "cmt_4f9d2a7b";
const publicBlockFromCommentParams: PublicBlockFromCommentParams = {
  reason: "spam",
  moderatorId: "mod_783",
  durationMinutes: 1440,
  notifyUser: true
};
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.example";
const result: BlockSuccess = await blockFromCommentPublic(tenantId, commentId, publicBlockFromCommentParams, sso);
[inline-code-end]