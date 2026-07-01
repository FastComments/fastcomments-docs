## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|------|
| commentId | string | 是 |  |
| tenantId | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回：[`GetCommentBanStatusResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentBanStatusResponse1.ts)

## 示例

[inline-code-attrs-start title = 'getCommentBanStatus 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function checkCommentBanStatus() {
  const banStatus: GetCommentBanStatusResponse1 = await getCommentBanStatus('cmt_987654321', 'tenant_42', 'sso_token_abc123');
  const banStatusNoTenant: GetCommentBanStatusResponse1 = await getCommentBanStatus('cmt_987654322', undefined, 'sso_token_def456');
}
[inline-code-end]