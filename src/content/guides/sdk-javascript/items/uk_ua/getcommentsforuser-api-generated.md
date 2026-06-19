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

Повертає: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getCommentsForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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