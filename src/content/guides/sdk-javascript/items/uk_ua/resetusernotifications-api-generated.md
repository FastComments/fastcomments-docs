## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| afterId | string | Ні |  |
| afterCreatedAt | number | Ні |  |
| unreadOnly | boolean | Ні |  |
| dmOnly | boolean | Ні |  |
| noDm | boolean | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ResetUserNotificationsResponse.ts)

## Приклад

[inline-code-attrs-start title = 'resetUserNotifications Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const afterId: string = "notif-2023-09-01";
  const afterCreatedAt: number = 1693526400; // Unix-мітка часу
  const unreadOnly: boolean = true;
  const dmOnly: boolean = false;

  const response: ResetUserNotificationsResponse = await resetUserNotifications(
    tenantId,
    afterId,
    afterCreatedAt,
    unreadOnly,
    dmOnly
  );

  console.log(response);
})();
[inline-code-end]

---