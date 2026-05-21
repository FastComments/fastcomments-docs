## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| userId | string | Не |  |
| tenantId | string | Не |  |
| urlId | string | Не |  |
| page | number | Не |  |
| direction | SortDirections | Не |  |
| lastGenDate | number | Не |  |
| repliesToUserId | string | Не |  |
| fetchPageForCommentId | string | Не |  |
| includei10n | boolean | Не |  |
| useFullTranslationIds | boolean | Не |  |
| locale | string | Не |  |
| includeConfig | boolean | Не |  |
| includeNotificationCount | boolean | Не |  |
| countAll | boolean | Не |  |
| sso | string | Не |  |

## Отговор

Връща: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getCommentsForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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