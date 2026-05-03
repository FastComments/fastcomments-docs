## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| userId | string | Non |  |
| tenantId | string | Non |  |
| urlId | string | Non |  |
| page | number | Non |  |
| direction | SortDirections | Non |  |
| lastGenDate | number | Non |  |
| repliesToUserId | string | Non |  |
| fetchPageForCommentId | string | Non |  |
| includei10n | boolean | Non |  |
| useFullTranslationIds | boolean | Non |  |
| locale | string | Non |  |
| includeConfig | boolean | Non |  |
| includeNotificationCount | boolean | Non |  |
| countAll | boolean | Non |  |
| sso | string | Non |  |

## Réponse

Retourne : [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getCommentsForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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