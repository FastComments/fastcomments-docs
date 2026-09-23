## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentId | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回: [`GetUserInternalProfileResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserInternalProfileResponse.ts)

## 示例

[inline-code-attrs-start title = 'getUserInternalProfile 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demo() {
  const tenantId: string = "tenant_12345";
  const commentId: string = "cmt_9876";
  const ssoToken: string = "sso_abcde12345";

  const profileOnly: GetUserInternalProfileResponse = await getUserInternalProfile(tenantId);
  const profileWithComment: GetUserInternalProfileResponse = await getUserInternalProfile(tenantId, commentId);
  const fullProfile: GetUserInternalProfileResponse = await getUserInternalProfile(tenantId, commentId, ssoToken);
}

demo();
[inline-code-end]

---