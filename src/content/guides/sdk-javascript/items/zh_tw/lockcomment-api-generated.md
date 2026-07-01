## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| broadcastId | string | 是 |  |
| sso | string | 否 |  |

## 回應

返回：[`LockCommentResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/LockCommentResponse.ts)

## 範例

[inline-code-attrs-start title = 'lockComment 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const commentId: string = "cmt_98765";
  const broadcastId: string = "brd_54321";

  // 使用可選的 SSO 令牌
  const ssoToken: string = "user-abc123";
  const lockedWithSso: LockCommentResponse = await lockComment(tenantId, commentId, broadcastId, ssoToken);

  // 未使用 SSO 令牌
  const lockedWithoutSso: LockCommentResponse = await lockComment(tenantId, commentId, broadcastId);
})();
[inline-code-end]