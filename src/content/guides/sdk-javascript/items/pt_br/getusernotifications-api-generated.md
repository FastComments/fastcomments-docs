## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| urlId | string | Não |  |
| pageSize | number | Não |  |
| afterId | string | Não |  |
| includeContext | boolean | Não |  |
| afterCreatedAt | number | Não |  |
| unreadOnly | boolean | Não |  |
| dmOnly | boolean | Não |  |
| noDm | boolean | Não |  |
| includeTranslations | boolean | Não |  |
| includeTenantNotifications | boolean | Não |  |
| sso | string | Não |  |

## Resposta

Retorna: [`GetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserNotificationsResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getUserNotifications'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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