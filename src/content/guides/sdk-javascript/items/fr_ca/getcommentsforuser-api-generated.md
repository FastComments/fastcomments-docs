---
## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| userId | string | Non |  |
| direction | SortDirections | Non |  |
| repliesToUserId | string | Non |  |
| page | number | Non |  |
| includei10n | boolean | Non |  |
| locale | string | Non |  |
| isCrawler | boolean | Non |  |

## Réponse

Renvoie : [`GetCommentsForUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUser200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple d\'utilisation de getCommentsForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const userId: string = "550e8400-e29b-41d4-a716-446655440000";
const page: number = 2;
const includei10n: boolean = true;
const locale: string = "en-US";
const isCrawler: boolean = false;

const comments: GetCommentsForUser200Response = await getCommentsForUser(
  userId,
  undefined, // direction omise
  undefined, // repliesToUserId omise
  page,
  includei10n,
  locale,
  isCrawler
);

console.log(comments);
[inline-code-end]

---