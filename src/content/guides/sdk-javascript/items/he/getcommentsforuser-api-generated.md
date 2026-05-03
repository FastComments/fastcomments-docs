## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| userId | string | לא |  |
| tenantId | string | לא |  |
| urlId | string | לא |  |
| page | number | לא |  |
| direction | SortDirections | לא |  |
| lastGenDate | number | לא |  |
| repliesToUserId | string | לא |  |
| fetchPageForCommentId | string | לא |  |
| includei10n | boolean | לא |  |
| useFullTranslationIds | boolean | לא |  |
| locale | string | לא |  |
| includeConfig | boolean | לא |  |
| includeNotificationCount | boolean | לא |  |
| countAll | boolean | לא |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getCommentsForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---