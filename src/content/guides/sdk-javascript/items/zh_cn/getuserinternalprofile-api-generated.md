## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|------|
| commentId | string | 否 |  |
| tenantId | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回：[`GetUserInternalProfileResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserInternalProfileResponse1.ts)

## 示例

[inline-code-attrs-start title = 'getUserInternalProfile 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const fullProfile: GetUserInternalProfileResponse1 = await getUserInternalProfile({
    commentId: "cmt_12345",
    tenantId: "tenant_67890",
    sso: "sso_token_abcdef"
  });

  const partialProfile: GetUserInternalProfileResponse1 = await getUserInternalProfile({
    commentId: "cmt_98765"
  });
})();
[inline-code-end]