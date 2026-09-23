## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| includeEmail | boolean | 否 |  |
| includeIP | boolean | 否 |  |
| sso | string | 否 |  |

## 回應

返回：[`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPICommentResponse.ts)

## 範例

[inline-code-attrs-start title = 'getModerationComment 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchComments() {
  const tenantId: string = "acme-corp";
  const commentId: string = "cmt_1234567890";

  // 僅使用必要參數呼叫
  const basicResponse: ModerationAPICommentResponse = await getModerationComment(tenantId, commentId);

  // 使用可選參數呼叫
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