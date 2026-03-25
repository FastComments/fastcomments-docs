## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| pageSize | number | 아니요 |  |
| afterId | string | 아니요 |  |
| includeContext | boolean | 아니요 |  |
| afterCreatedAt | number | 아니요 |  |
| unreadOnly | boolean | 아니요 |  |
| dmOnly | boolean | 아니요 |  |
| noDm | boolean | 아니요 |  |
| includeTranslations | boolean | 아니요 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserNotifications200Response.ts)

## 예제

[inline-code-attrs-start title = 'getUserNotifications 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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