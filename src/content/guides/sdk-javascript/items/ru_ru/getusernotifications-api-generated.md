## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Нет |  |
| pageSize | number | Нет |  |
| afterId | string | Нет |  |
| includeContext | boolean | Нет |  |
| afterCreatedAt | number | Нет |  |
| unreadOnly | boolean | Нет |  |
| dmOnly | boolean | Нет |  |
| noDm | boolean | Нет |  |
| includeTranslations | boolean | Нет |  |
| includeTenantNotifications | boolean | Нет |  |
| sso | string | Нет |  |

## Ответ

Возвращает: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetMyNotificationsResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример getUserNotifications'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_fc_92';
  const urlId: string = 'https://news.company.com/articles/2026/06/19/launch-update';
  const pageSize: number = 25;
  const afterId: string = 'notif_7890';
  const includeContext: boolean = true;
  const afterCreatedAt: number = Date.now() - 1000 * 60 * 60 * 24 * 7;
  const unreadOnly: boolean = false;
  const dmOnly: boolean = false;
  const noDm: boolean = false;
  const includeTranslations: boolean = true;
  const includeTenantNotifications: boolean = true;
  const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.signedPayload';
  const notifications: GetMyNotificationsResponse = await getUserNotifications(
    tenantId,
    urlId,
    pageSize,
    afterId,
    includeContext,
    afterCreatedAt,
    unreadOnly,
    dmOnly,
    noDm,
    includeTranslations,
    includeTenantNotifications,
    sso
  );
  console.log(notifications);
})();
[inline-code-end]

---