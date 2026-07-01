## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| urlId | string | Ne |  |
| pageSize | number | Ne |  |
| afterId | string | Ne |  |
| includeContext | boolean | Ne |  |
| afterCreatedAt | number | Ne |  |
| unreadOnly | boolean | Ne |  |
| dmOnly | boolean | Ne |  |
| noDm | boolean | Ne |  |
| includeTranslations | boolean | Ne |  |
| includeTenantNotifications | boolean | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`GetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserNotificationsResponse.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer getUserNotifications'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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