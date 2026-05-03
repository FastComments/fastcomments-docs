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