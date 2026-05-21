## Параметры

| Name | Type | Обязательный | Описание |
|------|------|--------------|----------|
| userId | string | Нет |  |
| tenantId | string | Нет |  |
| urlId | string | Нет |  |
| page | number | Нет |  |
| direction | SortDirections | Нет |  |
| lastGenDate | number | Нет |  |
| repliesToUserId | string | Нет |  |
| fetchPageForCommentId | string | Нет |  |
| includei10n | boolean | Нет |  |
| useFullTranslationIds | boolean | Нет |  |
| locale | string | Нет |  |
| includeConfig | boolean | Нет |  |
| includeNotificationCount | boolean | Нет |  |
| countAll | boolean | Нет |  |
| sso | string | Нет |  |

## Ответ

Возвращает: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример getCommentsForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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