## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| afterId | string | Не |  |
| afterCreatedAt | number | Не |  |
| unreadOnly | boolean | Не |  |
| dmOnly | boolean | Не |  |
| noDm | boolean | Не |  |
| sso | string | Не |  |

## Отговор

Връща: [`ResetUserNotificationsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ResetUserNotificationsResponse1.ts)

## Пример

[inline-code-attrs-start title = 'resetUserNotifications пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant-001";
  const afterId: string = "notif-123";
  const afterCreatedAt: number = 1697049600; // примерен UNIX timestamp
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