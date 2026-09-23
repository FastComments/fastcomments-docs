## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| userId | string | Nie |  |
| direction | SortDirections | Nie |  |
| repliesToUserId | string | Nie |  |
| page | number | Nie |  |
| includei10n | boolean | Nie |  |
| locale | string | Nie |  |
| isCrawler | boolean | Nie |  |

## Odpowiedź

Zwraca: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getCommentsForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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