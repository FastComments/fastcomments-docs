## Параметри

| Назва | Тип | Обов’язково | Опис |
|------|------|-------------|------|
| tenantId | string | Так |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`GetUserNotificationCountResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserNotificationCountResponse1.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getUserNotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoGetUserNotificationCount() {
    const tenantId: string = "acme-corp-01";

    // Виклик з необов’язковим токеном SSO
    const countWithSSO: GetUserNotificationCountResponse1 = await getUserNotificationCount(
        tenantId,
        "sso-token-abc123"
    );

    // Виклик без токена SSO
    const countWithoutSSO: GetUserNotificationCountResponse1 = await getUserNotificationCount(
        tenantId
    );

    console.log(countWithSSO, countWithoutSSO);
}
[inline-code-end]