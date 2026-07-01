## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| sso | string | No |  |

## レスポンス

戻り値: [`GetUserNotificationCountResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserNotificationCountResponse1.ts)

## 例

[inline-code-attrs-start title = 'getUserNotificationCount の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoGetUserNotificationCount() {
    const tenantId: string = "acme-corp-01";

    // オプションの SSO トークンで呼び出す
    const countWithSSO: GetUserNotificationCountResponse1 = await getUserNotificationCount(
        tenantId,
        "sso-token-abc123"
    );

    // SSO トークンなしで呼び出す
    const countWithoutSSO: GetUserNotificationCountResponse1 = await getUserNotificationCount(
        tenantId
    );

    console.log(countWithSSO, countWithoutSSO);
}
[inline-code-end]

---