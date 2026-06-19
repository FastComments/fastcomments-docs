## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| userId | string | לא |  |
| direction | SortDirections | לא |  |
| repliesToUserId | string | לא |  |
| page | number | לא |  |
| includei10n | boolean | לא |  |
| locale | string | לא |  |
| isCrawler | boolean | לא |  |

## תגובה

מחזיר: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getCommentsForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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