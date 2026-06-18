## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| userId | string | Ні |  |
| direction | SortDirections | Ні |  |
| repliesToUserId | string | Ні |  |
| page | number | Ні |  |
| includei10n | boolean | Ні |  |
| locale | string | Ні |  |
| isCrawler | boolean | Ні |  |

## Відповідь

Повертає: [`GetCommentsForUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUser200Response.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getCommentsForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const userId: string = "550e8400-e29b-41d4-a716-446655440000";
const page: number = 2;
const includei10n: boolean = true;
const locale: string = "en-US";
const isCrawler: boolean = false;

const comments: GetCommentsForUser200Response = await getCommentsForUser(
  userId,
  undefined, // direction пропущено
  undefined, // repliesToUserId пропущено
  page,
  includei10n,
  locale,
  isCrawler
);

console.log(comments);
[inline-code-end]

---