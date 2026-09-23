## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| sso | string | 否 |  |

## 回應

返回: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ResetUserNotificationsResponse.ts)

## 範例

[inline-code-attrs-start title = 'resetUserNotificationCount 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoReset() {
  const tenantId: string = "tenant_12345";
  const ssoToken: string = "sso_abcdef123456";

  const resultWithSso: ResetUserNotificationsResponse = await resetUserNotificationCount(tenantId, ssoToken);
  const resultWithoutSso: ResetUserNotificationsResponse = await resetUserNotificationCount(tenantId);
}

demoReset();
[inline-code-end]

---