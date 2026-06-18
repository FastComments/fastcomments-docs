## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-------------|
| userId | string | Não |  |
| direction | SortDirections | Não |  |
| repliesToUserId | string | Não |  |
| page | number | Não |  |
| includei10n | boolean | Não |  |
| locale | string | Não |  |
| isCrawler | boolean | Não |  |

## Resposta

Retorna: [`GetCommentsForUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUser200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getCommentsForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const userId: string = "550e8400-e29b-41d4-a716-446655440000";
const page: number = 2;
const includei10n: boolean = true;
const locale: string = "en-US";
const isCrawler: boolean = false;

const comments: GetCommentsForUser200Response = await getCommentsForUser(
  userId,
  undefined, // direction omitido
  undefined, // repliesToUserId omitido
  page,
  includei10n,
  locale,
  isCrawler
);

console.log(comments);
[inline-code-end]

---