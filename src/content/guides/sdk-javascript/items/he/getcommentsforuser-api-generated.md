## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| userId | string | לא |  |
| tenantId | string | לא |  |
| urlId | string | לא |  |
| page | number | לא |  |
| direction | SortDirections | לא |  |
| lastGenDate | number | לא |  |
| repliesToUserId | string | לא |  |
| fetchPageForCommentId | string | לא |  |
| includei10n | boolean | לא |  |
| useFullTranslationIds | boolean | לא |  |
| locale | string | לא |  |
| includeConfig | boolean | לא |  |
| includeNotificationCount | boolean | לא |  |
| countAll | boolean | לא |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getCommentsForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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