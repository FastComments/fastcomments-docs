## Parametri

| Ime | Tip | Zahtevano | Opis |
|------|------|----------|-------------|
| userId | string | Ne |  |
| direction | SortDirections | Ne |  |
| repliesToUserId | string | Ne |  |
| page | number | Ne |  |
| includei10n | boolean | Ne |  |
| locale | string | Ne |  |
| isCrawler | boolean | Ne |  |

## Odgovor

Vrne: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer getCommentsForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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