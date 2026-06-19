## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 아니요 |  |
| pageSize | number | 아니요 |  |
| afterId | string | 아니요 |  |
| includeContext | boolean | 아니요 |  |
| afterCreatedAt | number | 아니요 |  |
| unreadOnly | boolean | 아니요 |  |
| dmOnly | boolean | 아니요 |  |
| noDm | boolean | 아니요 |  |
| includeTranslations | boolean | 아니요 |  |
| includeTenantNotifications | boolean | 아니요 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetMyNotificationsResponse.ts)

## 예제

[inline-code-attrs-start title = 'getUserNotifications 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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