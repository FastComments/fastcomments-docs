## 參數

| 名稱 | 類型 | 必要 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 否 |  |
| pageSize | number | 否 |  |
| afterId | string | 否 |  |
| includeContext | boolean | 否 |  |
| afterCreatedAt | number | 否 |  |
| unreadOnly | boolean | 否 |  |
| dmOnly | boolean | 否 |  |
| noDm | boolean | 否 |  |
| includeTranslations | boolean | 否 |  |
| includeTenantNotifications | boolean | 否 |  |
| sso | string | 否 |  |

## 回應

返回：[`GetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserNotificationsResponse.ts)

## 範例

[inline-code-attrs-start title = 'getUserNotifications 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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