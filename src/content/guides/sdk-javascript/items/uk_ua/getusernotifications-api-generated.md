## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
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

## Відповідь

Повертає: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetMyNotificationsResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getUserNotifications'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchNotifications(): Promise<void> {
  const tenantId: string = "acme-corp";
  const urlId: string = "article-42";
  const pageSize: number = 20;
  const includeContext: boolean = true;
  const unreadOnly: boolean = true;

  const notifications: GetMyNotificationsResponse = await getUserNotifications(
    tenantId,
    urlId,
    pageSize,
    undefined,
    includeContext,
    undefined,
    unreadOnly
  );

  console.log(notifications);
}
[inline-code-end]

---