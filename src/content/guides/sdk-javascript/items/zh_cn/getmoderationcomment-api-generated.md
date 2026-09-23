## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| includeEmail | boolean | 否 |  |
| includeIP | boolean | 否 |  |
| sso | string | 否 |  |

## 响应

返回：[`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPICommentResponse.ts)

## 示例

[inline-code-attrs-start title = 'getModerationComment 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchComments() {
  const tenantId: string = "acme-corp";
  const commentId: string = "cmt_1234567890";

  // 仅使用必填参数调用
  const basicResponse: ModerationAPICommentResponse = await getModerationComment(tenantId, commentId);

  // 使用可选参数调用
  const includeEmail: boolean = true;
  const includeIP: boolean = false;
  const sso: string = "sso-token-abc123";
  const detailedResponse: ModerationAPICommentResponse = await getModerationComment(
    tenantId,
    commentId,
    includeEmail,
    includeIP,
    sso
  );
}
[inline-code-end]

---