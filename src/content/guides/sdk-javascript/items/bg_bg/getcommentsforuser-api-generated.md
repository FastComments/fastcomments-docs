---
## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| userId | string | Не |  |
| direction | SortDirections | Не |  |
| repliesToUserId | string | Не |  |
| page | number | Не |  |
| includei10n | boolean | Не |  |
| locale | string | Не |  |
| isCrawler | boolean | Не |  |

## Отговор

Връща: [`GetCommentsForUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUser200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getCommentsForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---