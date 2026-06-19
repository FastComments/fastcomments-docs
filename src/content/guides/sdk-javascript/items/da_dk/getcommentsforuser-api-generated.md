## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| userId | string | Nej |  |
| direction | SortDirections | Nej |  |
| repliesToUserId | string | Nej |  |
| page | number | Nej |  |
| includei10n | boolean | Nej |  |
| locale | string | Nej |  |
| isCrawler | boolean | Nej |  |

## Svar

Returnerer: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'getCommentsForUser Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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