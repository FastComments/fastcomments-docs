## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| sso | string | Не |  |

## Одговор

Враћа: [`GetUserNotificationCountResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserNotificationCountResponse1.ts)

## Пример

[inline-code-attrs-start title = 'Primer getUserNotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoGetUserNotificationCount() {
    const tenantId: string = "acme-corp-01";

    // Позив са опционо SSO токеном
    const countWithSSO: GetUserNotificationCountResponse1 = await getUserNotificationCount(
        tenantId,
        "sso-token-abc123"
    );

    // Позив без SSO токена
    const countWithoutSSO: GetUserNotificationCountResponse1 = await getUserCount(
        tenantId
    );

    console.log(countWithSSO, countWithoutSSO);
}
[inline-code-end]