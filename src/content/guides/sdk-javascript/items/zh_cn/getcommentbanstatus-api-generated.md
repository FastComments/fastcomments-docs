## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 是 |  |
| sso | string | 否 |  |

## 响应

返回: [`GetCommentBanStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentBanStatusResponse.ts)

## 示例

[inline-code-attrs-start title = 'getCommentBanStatus 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_12345";
  const commentId: string = "cmt_987654";
  const ssoToken: string = "sso_user_abc";

  const statusWithSso: GetCommentBanStatusResponse = await getCommentBanStatus(tenantId, commentId, ssoToken);
  const statusWithoutSso: GetCommentBanStatusResponse = await getCommentBanStatus(tenantId, commentId);
})();
[inline-code-end]