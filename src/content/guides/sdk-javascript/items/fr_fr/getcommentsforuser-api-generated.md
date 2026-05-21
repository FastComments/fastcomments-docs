---
## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| userId | string | Non |  |
| tenantId | string | Non |  |
| urlId | string | Non |  |
| page | number | Non |  |
| direction | SortDirections | Non |  |
| lastGenDate | number | Non |  |
| repliesToUserId | string | Non |  |
| fetchPageForCommentId | string | Non |  |
| includei10n | boolean | Non |  |
| useFullTranslationIds | boolean | Non |  |
| locale | string | Non |  |
| includeConfig | boolean | Non |  |
| includeNotificationCount | boolean | Non |  |
| countAll | boolean | Non |  |
| sso | string | Non |  |

## Réponse

Renvoie: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getCommentsForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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