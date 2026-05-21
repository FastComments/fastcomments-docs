## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| userId | string | Hayır |  |
| tenantId | string | Hayır |  |
| urlId | string | Hayır |  |
| page | number | Hayır |  |
| direction | SortDirections | Hayır |  |
| lastGenDate | number | Hayır |  |
| repliesToUserId | string | Hayır |  |
| fetchPageForCommentId | string | Hayır |  |
| includei10n | boolean | Hayır |  |
| useFullTranslationIds | boolean | Hayır |  |
| locale | string | Hayır |  |
| includeConfig | boolean | Hayır |  |
| includeNotificationCount | boolean | Hayır |  |
| countAll | boolean | Hayır |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getCommentsForUser Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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