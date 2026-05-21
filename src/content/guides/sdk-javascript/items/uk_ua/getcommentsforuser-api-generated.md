---
## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| userId | string | Ні |  |
| tenantId | string | Ні |  |
| urlId | string | Ні |  |
| page | number | Ні |  |
| direction | SortDirections | Ні |  |
| lastGenDate | number | Ні |  |
| repliesToUserId | string | Ні |  |
| fetchPageForCommentId | string | Ні |  |
| includei10n | boolean | Ні |  |
| useFullTranslationIds | boolean | Ні |  |
| locale | string | Ні |  |
| includeConfig | boolean | Ні |  |
| includeNotificationCount | boolean | Ні |  |
| countAll | boolean | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getCommentsForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const userId: string = 'user_82f9b';
const tenantId: string = 'tenant_22';
const page: number = 2;
const lastGenDate: number = Date.now();
const includei10n: boolean = true;
const useFullTranslationIds: boolean = false;
const locale: string = 'en-US';
const includeConfig: boolean = true;
const includeNotificationCount: boolean = true;
const countAll: boolean = false;
const sso: string = 'sso-token-1a2b';
const commentsResponse: GetCommentsForUserResponse = await getCommentsForUser(userId, tenantId, undefined, page, undefined, lastGenDate, undefined, undefined, includei10n, useFullTranslationIds, locale, includeConfig, includeNotificationCount, countAll, sso);
[inline-code-end]

---