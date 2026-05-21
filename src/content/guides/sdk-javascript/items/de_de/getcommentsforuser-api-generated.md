## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| userId | string | Nein |  |
| tenantId | string | Nein |  |
| urlId | string | Nein |  |
| page | number | Nein |  |
| direction | SortDirections | Nein |  |
| lastGenDate | number | Nein |  |
| repliesToUserId | string | Nein |  |
| fetchPageForCommentId | string | Nein |  |
| includei10n | boolean | Nein |  |
| useFullTranslationIds | boolean | Nein |  |
| locale | string | Nein |  |
| includeConfig | boolean | Nein |  |
| includeNotificationCount | boolean | Nein |  |
| countAll | boolean | Nein |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getCommentsForUser Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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