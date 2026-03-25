---
## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| sso | string | 否 |  |

## 回應

回傳: [`ResetUserNotifications200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ResetUserNotifications200Response.ts)

## 範例

[inline-code-attrs-start title = 'resetUserNotificationCount 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_9f3b2c4a";
  const ssoToken: string | undefined = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9._sample_payload_.signature";
  const responseWithSSO: ResetUserNotifications200Response = await resetUserNotificationCount(tenantId, ssoToken);
  const responseWithoutSSO: ResetUserNotifications200Response = await resetUserNotificationCount(tenantId);
  console.log(responseWithSSO, responseWithoutSSO);
})();
[inline-code-end]

---