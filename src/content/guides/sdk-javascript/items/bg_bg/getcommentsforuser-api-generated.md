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

Връща: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getCommentsForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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