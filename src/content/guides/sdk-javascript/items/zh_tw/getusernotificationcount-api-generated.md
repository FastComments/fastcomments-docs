## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| sso | string | No |  |

## 回應

返回：[`GetUserNotificationCountResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserNotificationCountResponse1.ts)

## 範例

[inline-code-attrs-start title = 'getUserNotificationCount 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoGetUserNotificationCount() {
    const tenantId: string = "acme-corp-01";

    // Call with optional SSO token
    const countWithSSO: GetUserNotificationCountResponse1 = await getUserNotificationCount(
        tenantId,
        "sso-token-abc123"
    );

    // Call without SSO token
    const countWithoutSSO: GetUserNotificationCountResponse1 = await getUserNotificationCount(
        tenantId
    );

    console.log(countWithSSO, countWithoutSSO);
}
[inline-code-end]