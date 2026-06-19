## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| userId | string | Non |  |
| direction | SortDirections | Non |  |
| repliesToUserId | string | Non |  |
| page | number | Non |  |
| includei10n | boolean | Non |  |
| locale | string | Non |  |
| isCrawler | boolean | Non |  |

## Réponse

Renvoie : [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getCommentsForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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