## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| userId | string | No |  |
| direction | SortDirections | No |  |
| repliesToUserId | string | No |  |
| page | number | No |  |
| includei10n | boolean | No |  |
| locale | string | No |  |
| isCrawler | boolean | No |  |

## Respuesta

Devuelve: [`GetCommentsForUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUser200Response.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getCommentsForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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