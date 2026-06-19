## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| urlId | string | לא |  |
| pageSize | number | לא |  |
| afterId | string | לא |  |
| includeContext | boolean | לא |  |
| afterCreatedAt | number | לא |  |
| unreadOnly | boolean | לא |  |
| dmOnly | boolean | לא |  |
| noDm | boolean | לא |  |
| includeTranslations | boolean | לא |  |
| includeTenantNotifications | boolean | לא |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetMyNotificationsResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getUserNotifications'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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