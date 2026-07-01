## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | 是 |  |
| tenantId | string | 否 |  |
| sso | string | 否 |  |

## 回應

Returns: [`GetCommentBanStatusResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentBanStatusResponse1.ts)

## 範例

[inline-code-attrs-start title = 'getCommentBanStatus 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function checkCommentBanStatus() {
  const banStatus: GetCommentBanStatusResponse1 = await getCommentBanStatus('cmt_987654321', 'tenant_42', 'sso_token_abc123');
  const banStatusNoTenant: GetCommentBanStatusResponse1 = await getCommentBanStatus('cmt_987654322', undefined, 'sso_token_def456');
}
[inline-code-end]

---