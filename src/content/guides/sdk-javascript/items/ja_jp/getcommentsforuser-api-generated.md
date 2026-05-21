## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| userId | string | いいえ |  |
| tenantId | string | いいえ |  |
| urlId | string | いいえ |  |
| page | number | いいえ |  |
| direction | SortDirections | いいえ |  |
| lastGenDate | number | いいえ |  |
| repliesToUserId | string | いいえ |  |
| fetchPageForCommentId | string | いいえ |  |
| includei10n | boolean | いいえ |  |
| useFullTranslationIds | boolean | いいえ |  |
| locale | string | いいえ |  |
| includeConfig | boolean | いいえ |  |
| includeNotificationCount | boolean | いいえ |  |
| countAll | boolean | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

返却: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse.ts)

## 例

[inline-code-attrs-start title = 'getCommentsForUser の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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