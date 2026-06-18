---
## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|------|
| tenantId | string | 是 |  |
| urlId | string | 否 |  |
| pageSize | number | 否 |  |
| afterId | string | 否 |  |
| includeContext | boolean | 否 |  |
| afterCreatedAt | number | 否 |  |
| unreadOnly | boolean | 否 |  |
| dmOnly | boolean | 否 |  |
| noDm | boolean | 否 |  |
| includeTranslations | boolean | 否 |  |
| includeTenantNotifications | boolean | 否 |  |
| sso | string | 否 |  |

## 响应

返回: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserNotifications200Response.ts)

## 示例

[inline-code-attrs-start title = 'getUserNotifications 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---