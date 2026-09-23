## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| sso | string | 否 |  |

## 响应

返回：[`APIModerateGetUserBanPreferencesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIModerateGetUserBanPreferencesResponse.ts)

## 示例

[inline-code-attrs-start title = 'getUserBanPreference 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoGetUserBanPreference() {
  const tenantId: string = "tenant_987654321";
  const ssoToken: string = "sso_user_abc123xyz";

  const responseWithSso: APIModerateGetUserBanPreferencesResponse = await getUserBanPreference(tenantId, ssoToken);
  const responseWithoutSso: APIModerateGetUserBanPreferencesResponse = await getUserBanPreference(tenantId);
}
[inline-code-end]