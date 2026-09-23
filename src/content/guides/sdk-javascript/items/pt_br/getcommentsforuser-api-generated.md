## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| userId | string | Não |  |
| direction | SortDirections | Não |  |
| repliesToUserId | string | Não |  |
| page | number | Não |  |
| includei10n | boolean | Não |  |
| locale | string | Não |  |
| isCrawler | boolean | Não |  |

## Resposta

Retorna: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getCommentsForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const userId: string = '550e8400-e29b-41d4-a716-446655440000';
const direction: SortDirections = SortDirections.Descending;
const page: number = 3;
const includei10n: boolean = true;
const locale: string = 'fr-FR';
const isCrawler: boolean = false;

const commentsResponse: GetCommentsForUserResponse = await getCommentsForUser(
  userId,
  direction,
  undefined,
  page,
  includei10n,
  locale,
  isCrawler
);
[inline-code-end]