## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|-------------|------|
| tenantId | string | Yes |  |
| afterId | string | No |  |
| afterCreatedAt | number | No |  |
| unreadOnly | boolean | No |  |
| dmOnly | boolean | No |  |
| noDm | boolean | No |  |
| sso | string | No |  |

## Відповідь

Повертає: [`ResetUserNotificationsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ResetUserNotificationsResponse1.ts)

## Приклад

[inline-code-attrs-start title = 'resetUserNotifications Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant-001";
  const afterId: string = "notif-123";
  const afterCreatedAt: number = 1697049600; // приклад UNIX timestamp
  const unreadOnly: boolean = true;
  const dmOnly: boolean = false;
  const noDm: boolean = false;
  const sso: string = "sso-token-xyz";

  const result: ResetUserNotificationsResponse1 = await resetUserNotifications(
    tenantId,
    afterId,
    afterCreatedAt,
    unreadOnly,
    dmOnly,
    noDm,
    sso
  );

  console.log(result);
})();
[inline-code-end]