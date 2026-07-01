## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| urlId | string | No |  |
| pageSize | number | No |  |
| afterId | string | No |  |
| includeContext | boolean | No |  |
| afterCreatedAt | number | No |  |
| unreadOnly | boolean | No |  |
| dmOnly | boolean | No |  |
| noDm | boolean | No |  |
| includeTranslations | boolean | No |  |
| includeTenantNotifications | boolean | No |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`GetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserNotificationsResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo getUserNotifications'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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