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

반환: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserNotifications200Response.ts)

## 예제

[inline-code-attrs-start title = 'getUserNotifications 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_8271";
const urlId: string = "https://www.news-site.com/articles/2026/06/15/ai-updates";
const pageSize: number = 25;
const afterId: string = "notif_abc123";
const includeContext: boolean = true;
const afterCreatedAt: number = Date.now() - 86_400_000;
const unreadOnly: boolean = true;
const dmOnly: boolean = false;
const noDm: boolean = false;
const includeTranslations: boolean = true;
const includeTenantNotifications: boolean = true;
const sso: string = "sso_token_xyz_987";

const notifications: GetUserNotifications200Response = await getUserNotifications(
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
[inline-code-end]