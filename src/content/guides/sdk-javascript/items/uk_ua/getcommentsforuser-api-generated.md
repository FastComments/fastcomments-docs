## Параметри

| Назва | Тип | Обов’язково | Опис |
|------|------|-------------|------|
| userId | string | Ні |  |
| direction | SortDirections | Ні |  |
| repliesToUserId | string | Ні |  |
| page | number | Ні |  |
| includei10n | boolean | Ні |  |
| locale | string | Ні |  |
| isCrawler | boolean | Ні |  |

## Відповідь

Повертає: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getCommentsForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const userId: string = '550e8400-e29b-41d4-a716-446655440000';
const direction: SortDirections = SortDirections.Descending;
const page: number = 3;
const includei10n: boolean = true;
const locale: string = 'fr-FR';
const isCrawler: boolean = false;

const commentsResponse: GetCommentsForUserResponse = await getCommentsForUser(
  userId,
  direction,
  undefined,
  page,
  includei10n,
  locale,
  isCrawler
);
[inline-code-end]

---