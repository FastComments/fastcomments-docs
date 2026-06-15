## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| userId | string | Nee |  |
| direction | SortDirections | Nee |  |
| repliesToUserId | string | Nee |  |
| page | number | Nee |  |
| includei10n | boolean | Nee |  |
| locale | string | Nee |  |
| isCrawler | boolean | Nee |  |

## Respons

Retourneert: [`GetCommentsForUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUser200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getCommentsForUser Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const userId: string = "550e8400-e29b-41d4-a716-446655440000";
const page: number = 2;
const includei10n: boolean = true;
const locale: string = "en-US";
const isCrawler: boolean = false;

const comments: GetCommentsForUser200Response = await getCommentsForUser(
  userId,
  undefined, // direction omitted
  undefined, // repliesToUserId omitted
  page,
  includei10n,
  locale,
  isCrawler
);

console.log(comments);
[inline-code-end]