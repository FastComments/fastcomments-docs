## 參數

| 名稱 | 型別 | 必填 | 說明 |
|------|------|------|-------------|
| userId | string | 否 |  |
| tenantId | string | 否 |  |
| urlId | string | 否 |  |
| page | number | 否 |  |
| direction | SortDirections | 否 |  |
| lastGenDate | number | 否 |  |
| repliesToUserId | string | 否 |  |
| fetchPageForCommentId | string | 否 |  |
| includei10n | boolean | 否 |  |
| useFullTranslationIds | boolean | 否 |  |
| locale | string | 否 |  |
| includeConfig | boolean | 否 |  |
| includeNotificationCount | boolean | 否 |  |
| countAll | boolean | 否 |  |
| sso | string | 否 |  |

## 回應

回傳: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse.ts)

## 範例

[inline-code-attrs-start title = 'getCommentsForUser 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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