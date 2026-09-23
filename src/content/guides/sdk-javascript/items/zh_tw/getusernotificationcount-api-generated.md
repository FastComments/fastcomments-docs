## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| sso | string | 否 |  |

## 回應

返回：[`GetUserNotificationCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserNotificationCountResponse.ts)

## 範例

[inline-code-attrs-start title = 'getUserNotificationCount 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demo(): Promise<void> {
  const tenantId: string = "acme-corp-001";
  const ssoToken: string = "sso-token-xyz789";

  const countWithoutSso: GetUserNotificationCountResponse = await getUserNotificationCount(tenantId);
  const countWithSso: GetUserNotificationCountResponse = await getUserNotificationCount(tenantId, ssoToken);
}

demo();
[inline-code-end]

---