## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Не |  |
| pageSize | number | Не |  |
| afterId | string | Не |  |
| includeContext | boolean | Не |  |
| afterCreatedAt | number | Не |  |
| unreadOnly | boolean | Не |  |
| dmOnly | boolean | Не |  |
| noDm | boolean | Не |  |
| includeTranslations | boolean | Не |  |
| includeTenantNotifications | boolean | Не |  |
| sso | string | Не |  |

## Отговор

Връща: [`GetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserNotificationsResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getUserNotifications'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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