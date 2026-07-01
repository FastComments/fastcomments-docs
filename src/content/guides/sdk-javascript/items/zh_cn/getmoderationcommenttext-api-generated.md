## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| commentId | string | 是 |  |
| tenantId | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回：[`GetModerationCommentTextResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerationCommentTextResponse.ts)

## 示例

[inline-code-attrs-start title = 'getModerationCommentText 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function exampleUsage(): Promise<void> {
  const commentId: string = "cmt_9f8e7d6c5b4a3b2c1d0e";
  const tenantId: string = "tenant_67890";
  const sso: string = "sso_token_abc123";

  // 仅使用必需参数进行调用
  const result1: GetModerationCommentTextResponse = await getModerationCommentText(commentId);

  // 使用可选参数进行调用
  const result2: GetModerationCommentTextResponse = await getModerationCommentText(commentId, tenantId, sso);
}
[inline-code-end]

---