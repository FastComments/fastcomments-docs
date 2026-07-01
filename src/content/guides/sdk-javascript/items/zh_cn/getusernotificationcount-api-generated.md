## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| sso | string | No |  |

## 响应

Returns: [`GetUserNotificationCountResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserNotificationCountResponse1.ts)

## 示例

[inline-code-attrs-start title = 'getUserNotificationCount 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoGetUserNotificationCount() {
    const tenantId: string = "acme-corp-01";

    // 使用可选的 SSO 令牌调用
    const countWithSSO: GetUserNotificationCountResponse1 = await getUserNotificationCount(
        tenantId,
        "sso-token-abc123"
    );

    // 调用时不使用 SSO 令牌
    const countWithoutSSO: GetUserNotificationCountResponse1 = await getUserNotificationCount(
        tenantId
    );

    console.log(countWithSSO, countWithoutSSO);
}
[inline-code-end]