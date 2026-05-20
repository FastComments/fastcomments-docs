## Parameters

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

## Response

Returns: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse.ts)

## Example

[inline-code-attrs-start title = 'getCommentsForUser Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const userId: string = 'user_12345', tenantId: string = 'tenant_acme', urlId: string = 'https://news.example.com/article/987';
const page: number = 1, lastGenDate: number = Date.now() - 60 * 60 * 1000;
const includei10n: boolean = true, useFullTranslationIds: boolean = false, locale: string = 'en-US';
const includeConfig: boolean = true, includeNotificationCount: boolean = false, countAll: boolean = false, sso: string = 'sso-token-abc123';
const response: GetCommentsForUserResponse = await getCommentsForUser(userId, tenantId, urlId, page, undefined, lastGenDate, undefined, undefined, includei10n, useFullTranslationIds, locale, includeConfig, includeNotificationCount, countAll, sso);
[inline-code-end]
