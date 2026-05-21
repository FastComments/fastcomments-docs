## Parametry

| Name | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| userId | string | Nie |  |
| tenantId | string | Nie |  |
| urlId | string | Nie |  |
| page | number | Nie |  |
| direction | SortDirections | Nie |  |
| lastGenDate | number | Nie |  |
| repliesToUserId | string | Nie |  |
| fetchPageForCommentId | string | Nie |  |
| includei10n | boolean | Nie |  |
| useFullTranslationIds | boolean | Nie |  |
| locale | string | Nie |  |
| includeConfig | boolean | Nie |  |
| includeNotificationCount | boolean | Nie |  |
| countAll | boolean | Nie |  |
| sso | string | Nie |  |

## Odpowiedź

Zwraca: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getCommentsForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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