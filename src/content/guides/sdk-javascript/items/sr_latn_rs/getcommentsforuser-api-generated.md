## Parametri

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

## Odgovor

Vraća: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse.ts)

## Primer

[inline-code-attrs-start title = 'getCommentsForUser Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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