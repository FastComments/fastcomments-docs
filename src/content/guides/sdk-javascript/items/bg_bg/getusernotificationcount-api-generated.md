## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| sso | string | Не |  |

## Отговор

Връща: [`GetUserNotificationCountResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserNotificationCountResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getUserNotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demo(): Promise<void> {
  const tenantId: string = "acme-corp-001";
  const ssoToken: string = "sso-token-xyz789";

  const countWithoutSso: GetUserNotificationCountResponse = await getUserNotificationCount(tenantId);
  const countWithSso: GetUserNotificationCountResponse = await getUserNotificationCount(tenantId, ssoToken);
}

demo();
[inline-code-end]

---