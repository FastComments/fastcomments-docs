## Parametry

| Name | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| userId | string | Nie |  |
| direction | SortDirections | Nie |  |
| repliesToUserId | string | Nie |  |
| page | number | Nie |  |
| includei10n | boolean | Nie |  |
| locale | string | Nie |  |
| isCrawler | boolean | Nie |  |

## Odpowiedź

Zwraca: [`GetCommentsForUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUser200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład użycia getCommentsForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const userId: string = "550e8400-e29b-41d4-a716-446655440000";
const page: number = 2;
const includei10n: boolean = true;
const locale: string = "en-US";
const isCrawler: boolean = false;

const comments: GetCommentsForUser200Response = await getCommentsForUser(
  userId,
  undefined, // direction pominięto
  undefined, // repliesToUserId pominięto
  page,
  includei10n,
  locale,
  isCrawler
);

console.log(comments);
[inline-code-end]