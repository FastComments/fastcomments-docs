## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| urlId | string | Hayır |  |
| pageSize | number | Hayır |  |
| afterId | string | Hayır |  |
| includeContext | boolean | Hayır |  |
| afterCreatedAt | number | Hayır |  |
| unreadOnly | boolean | Hayır |  |
| dmOnly | boolean | Hayır |  |
| noDm | boolean | Hayır |  |
| includeTranslations | boolean | Hayır |  |
| includeTenantNotifications | boolean | Hayır |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`GetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserNotificationsResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getUserNotifications Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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