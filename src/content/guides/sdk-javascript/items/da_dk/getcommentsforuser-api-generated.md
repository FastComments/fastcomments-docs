## Parametre

| Name | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| userId | string | Nej |  |
| tenantId | string | Nej |  |
| urlId | string | Nej |  |
| page | number | Nej |  |
| direction | SortDirections | Nej |  |
| lastGenDate | number | Nej |  |
| repliesToUserId | string | Nej |  |
| fetchPageForCommentId | string | Nej |  |
| includei10n | boolean | Nej |  |
| useFullTranslationIds | boolean | Nej |  |
| locale | string | Nej |  |
| includeConfig | boolean | Nej |  |
| includeNotificationCount | boolean | Nej |  |
| countAll | boolean | Nej |  |
| sso | string | Nej |  |

## Svar

Returnerer: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'getCommentsForUser Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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