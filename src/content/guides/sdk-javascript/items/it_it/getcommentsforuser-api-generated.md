## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| userId | string | No |  |
| tenantId | string | No |  |
| urlId | string | No |  |
| page | number | No |  |
| direction | SortDirections | No |  |
| lastGenDate | number | No |  |
| repliesToUserId | string | No |  |
| fetchPageForCommentId | string | No |  |
| includei10n | boolean | No |  |
| useFullTranslationIds | boolean | No |  |
| locale | string | No |  |
| includeConfig | boolean | No |  |
| includeNotificationCount | boolean | No |  |
| countAll | boolean | No |  |
| sso | string | No |  |

## Risposta

Restituisce: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di getCommentsForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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