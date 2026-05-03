## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| userId | string | Nein |  |
| tenantId | string | Nein |  |
| urlId | string | Nein |  |
| page | number | Nein |  |
| direction | SortDirections | Nein |  |
| lastGenDate | number | Nein |  |
| repliesToUserId | string | Nein |  |
| fetchPageForCommentId | string | Nein |  |
| includei10n | boolean | Nein |  |
| useFullTranslationIds | boolean | Nein |  |
| locale | string | Nein |  |
| includeConfig | boolean | Nein |  |
| includeNotificationCount | boolean | Nein |  |
| countAll | boolean | Nein |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getCommentsForUser Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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