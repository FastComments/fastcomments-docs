## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| urlId | string | Ні |  |
| pageSize | number | Ні |  |
| afterId | string | Ні |  |
| includeContext | boolean | Ні |  |
| afterCreatedAt | number | Ні |  |
| unreadOnly | boolean | Ні |  |
| dmOnly | boolean | Ні |  |
| noDm | boolean | Ні |  |
| includeTranslations | boolean | Ні |  |
| includeTenantNotifications | boolean | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`GetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserNotificationsResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getUserNotifications'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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