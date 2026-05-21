## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| userId | string | Όχι |  |
| tenantId | string | Όχι |  |
| urlId | string | Όχι |  |
| page | number | Όχι |  |
| direction | SortDirections | Όχι |  |
| lastGenDate | number | Όχι |  |
| repliesToUserId | string | Όχι |  |
| fetchPageForCommentId | string | Όχι |  |
| includei10n | boolean | Όχι |  |
| useFullTranslationIds | boolean | Όχι |  |
| locale | string | Όχι |  |
| includeConfig | boolean | Όχι |  |
| includeNotificationCount | boolean | Όχι |  |
| countAll | boolean | Όχι |  |
| sso | string | Όχι |  |

## Απάντηση

Επιστρέφει: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getCommentsForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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