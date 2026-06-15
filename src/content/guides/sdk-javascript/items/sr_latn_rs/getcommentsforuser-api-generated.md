## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| userId | string | Ne |  |
| direction | SortDirections | Ne |  |
| repliesToUserId | string | Ne |  |
| page | number | Ne |  |
| includei10n | boolean | Ne |  |
| locale | string | Ne |  |
| isCrawler | boolean | Ne |  |

## Odgovor

Vraća: [`GetCommentsForUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUser200Response.ts)

## Primer

[inline-code-attrs-start title = 'Primer getCommentsForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const userId: string = "550e8400-e29b-41d4-a716-446655440000";
const page: number = 2;
const includei10n: boolean = true;
const locale: string = "en-US";
const isCrawler: boolean = false;

const comments: GetCommentsForUser200Response = await getCommentsForUser(
  userId,
  undefined, // direction izostavljeno
  undefined, // repliesToUserId izostavljeno
  page,
  includei10n,
  locale,
  isCrawler
);

console.log(comments);
[inline-code-end]