---
## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| userId | string | 아니오 |  |
| tenantId | string | 아니오 |  |
| urlId | string | 아니오 |  |
| page | number | 아니오 |  |
| direction | SortDirections | 아니오 |  |
| lastGenDate | number | 아니오 |  |
| repliesToUserId | string | 아니오 |  |
| fetchPageForCommentId | string | 아니오 |  |
| includei10n | boolean | 아니오 |  |
| useFullTranslationIds | boolean | 아니오 |  |
| locale | string | 아니오 |  |
| includeConfig | boolean | 아니오 |  |
| includeNotificationCount | boolean | 아니오 |  |
| countAll | boolean | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse.ts)

## 예제

[inline-code-attrs-start title = 'getCommentsForUser 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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