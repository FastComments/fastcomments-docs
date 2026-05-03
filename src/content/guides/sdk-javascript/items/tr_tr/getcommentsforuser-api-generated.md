## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| userId | string | Hayır |  |
| tenantId | string | Hayır |  |
| urlId | string | Hayır |  |
| page | number | Hayır |  |
| direction | SortDirections | Hayır |  |
| lastGenDate | number | Hayır |  |
| repliesToUserId | string | Hayır |  |
| fetchPageForCommentId | string | Hayır |  |
| includei10n | boolean | Hayır |  |
| useFullTranslationIds | boolean | Hayır |  |
| locale | string | Hayır |  |
| includeConfig | boolean | Hayır |  |
| includeNotificationCount | boolean | Hayır |  |
| countAll | boolean | Hayır |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getCommentsForUser Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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