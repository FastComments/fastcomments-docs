## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| userId | string | Ne |  |
| tenantId | string | Ne |  |
| urlId | string | Ne |  |
| page | number | Ne |  |
| direction | SortDirections | Ne |  |
| lastGenDate | number | Ne |  |
| repliesToUserId | string | Ne |  |
| fetchPageForCommentId | string | Ne |  |
| includei10n | boolean | Ne |  |
| useFullTranslationIds | boolean | Ne |  |
| locale | string | Ne |  |
| includeConfig | boolean | Ne |  |
| includeNotificationCount | boolean | Ne |  |
| countAll | boolean | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vrne: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer getCommentsForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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