## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| urlId | string | Όχι |  |
| pageSize | number | Όχι |  |
| afterId | string | Όχι |  |
| includeContext | boolean | Όχι |  |
| afterCreatedAt | number | Όχι |  |
| unreadOnly | boolean | Όχι |  |
| dmOnly | boolean | Όχι |  |
| noDm | boolean | Όχι |  |
| includeTranslations | boolean | Όχι |  |
| includeTenantNotifications | boolean | Όχι |  |
| sso | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserNotifications200Response.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getUserNotifications'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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