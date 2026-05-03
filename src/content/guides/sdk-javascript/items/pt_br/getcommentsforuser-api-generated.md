## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| userId | string | Não |  |
| tenantId | string | Não |  |
| urlId | string | Não |  |
| page | number | Não |  |
| direction | SortDirections | Não |  |
| lastGenDate | number | Não |  |
| repliesToUserId | string | Não |  |
| fetchPageForCommentId | string | Não |  |
| includei10n | boolean | Não |  |
| useFullTranslationIds | boolean | Não |  |
| locale | string | Não |  |
| includeConfig | boolean | Não |  |
| includeNotificationCount | boolean | Não |  |
| countAll | boolean | Não |  |
| sso | string | Não |  |

## Resposta

Retorna: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getCommentsForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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