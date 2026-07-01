## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Tak |  |
| urlId | string | Nie |  |
| pageSize | number | Nie |  |
| afterId | string | Nie |  |
| includeContext | boolean | Nie |  |
| afterCreatedAt | number | Nie |  |
| unreadOnly | boolean | Nie |  |
| dmOnly | boolean | Nie |  |
| noDm | boolean | Nie |  |
| includeTranslations | boolean | Nie |  |
| includeTenantNotifications | boolean | Nie |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`GetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserNotificationsResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getUserNotifications'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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