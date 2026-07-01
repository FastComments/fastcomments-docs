## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| commentId | string | 是 |  |
| broadcastId | string | 否 |  |
| tenantId | string | 否 |  |
| sso | string | 否 |  |

## 回應

返回：[`PostFlagCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PostFlagCommentResponse.ts)

## 範例

[inline-code-attrs-start title = 'postFlagComment 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const commentId: string = "cmt_20230915_001";
const broadcastId: string = "brd_20230915_live";
const tenantId: string = "tenant_42";
const sso: string = "sso_token_abc123";

const flaggedResponse: PostFlagCommentResponse = await postFlagComment(
  commentId,
  broadcastId,
  tenantId,
  sso
);
[inline-code-end]