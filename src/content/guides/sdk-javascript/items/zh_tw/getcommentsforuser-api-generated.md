## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
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
const userId: string = "user_92b7f4";
const tenantId: string = "news-tenant-uk";
const urlId: string = "https://news.example.co.uk/articles/2026/05/01/local-election";
const page: number = 1;
const lastGenDate: number = Date.now() - 24 * 60 * 60 * 1000;
const fetchPageForCommentId: string = "c_987654321";
const includei10n: boolean = true;
const locale: string = "en-GB";
const includeConfig: boolean = true;
const includeNotificationCount: boolean = false;
const result: GetCommentsForUserResponse = await getCommentsForUser(
  userId,
  tenantId,
  urlId,
  page,
  undefined,
  lastGenDate,
  undefined,
  fetchPageForCommentId,
  includei10n,
  false,
  locale,
  includeConfig,
  includeNotificationCount,
  false,
  undefined
);
[inline-code-end]