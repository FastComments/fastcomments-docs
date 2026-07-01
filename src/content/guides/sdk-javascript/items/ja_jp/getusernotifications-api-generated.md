## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| urlId | string | いいえ |  |
| pageSize | number | いいえ |  |
| afterId | string | いいえ |  |
| includeContext | boolean | いいえ |  |
| afterCreatedAt | number | いいえ |  |
| unreadOnly | boolean | いいえ |  |
| dmOnly | boolean | いいえ |  |
| noDm | boolean | いいえ |  |
| includeTranslations | boolean | いいえ |  |
| includeTenantNotifications | boolean | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

返却: [`GetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserNotificationsResponse.ts)

## 例

[inline-code-attrs-start title = 'getUserNotifications の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchUserNotifications() {
    const tenantId: string = "tenant_9f4b2c";
    const urlId: string = "post_1234";
    const pageSize: number = 25;
    const afterId: string = "notif_5678";
    const includeContext: boolean = true;
    const unreadOnly: boolean = false;
    const dmOnly: boolean = false;
    const includeTranslations: boolean = true;

    const notifications: GetUserNotificationsResponse = await getUserNotifications(
        tenantId,
        urlId,
        pageSize,
        afterId,
        includeContext,
        undefined,
        unreadOnly,
        dmOnly,
        undefined,
        includeTranslations,
        undefined,
        undefined
    );

    console.log(notifications);
}
[inline-code-end]

---