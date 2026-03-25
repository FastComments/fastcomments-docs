## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| pageSize | number | Hayır |  |
| afterId | string | Hayır |  |
| includeContext | boolean | Hayır |  |
| afterCreatedAt | number | Hayır |  |
| unreadOnly | boolean | Hayır |  |
| dmOnly | boolean | Hayır |  |
| noDm | boolean | Hayır |  |
| includeTranslations | boolean | Hayır |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserNotifications200Response.ts)

## Örnek

[inline-code-attrs-start title = 'getUserNotifications Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f3b1c';
const pageSize: number = 25;
const afterId: string = 'notif_b2f9e4';
const includeContext: boolean = true;
const afterCreatedAt: number = Date.now() - 24 * 60 * 60 * 1000;
const unreadOnly: boolean = true;
const dmOnly: boolean = false;
const noDm: boolean = false;
const includeTranslations: boolean = true;
const sso: string = 'sso_tok_user_9f8d7c';
const response: GetUserNotifications200Response = await getUserNotifications(
  tenantId,
  pageSize,
  afterId,
  includeContext,
  afterCreatedAt,
  unreadOnly,
  dmOnly,
  noDm,
  includeTranslations,
  sso
);
[inline-code-end]

---