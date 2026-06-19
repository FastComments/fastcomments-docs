## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
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
(async () => {
  const userId: string = 'user_7421';
  const direction: SortDirections = SortDirections.Newest;
  const page: number = 2;
  const includei10n: boolean = true;
  const locale: string = 'en-GB';
  const isCrawler: boolean = false;
  const response: GetCommentsForUserResponse = await getCommentsForUser(userId, direction, undefined, page, includei10n, locale, isCrawler);
  console.log(response);
})();
[inline-code-end]

---