## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| commentId | string | 是 |  |
| includeEmail | boolean | 否 |  |
| includeIP | boolean | 否 |  |
| tenantId | string | 否 |  |
| sso | string | 否 |  |

## 回應

返回：[`GetModerationCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetModerationCommentResponse.ts)

## 範例

[inline-code-attrs-start title = 'getModerationComment 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchCommentDetails() {
  // 完整參數集合
  const commentId: string = "cmt_12345abc";
  const includeEmail: boolean = true;
  const includeIP: boolean = false;
  const tenantId: string = "tenant_9876";
  const sso: string = "sso_token_xyz";

  const fullResult: GetModerationCommentResponse = await getModerationComment(
    commentId,
    includeEmail,
    includeIP,
    tenantId,
    sso
  );

  // 只使用必要參數的最小呼叫
  const minimalResult: GetModerationCommentResponse = await getModerationComment("cmt_67890def");

  // 根據需要使用結果...
}
[inline-code-end]