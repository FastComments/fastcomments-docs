## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| userId | string | לא |  |
| direction | SortDirections | לא |  |
| repliesToUserId | string | לא |  |
| page | number | לא |  |
| includei10n | boolean | לא |  |
| locale | string | לא |  |
| isCrawler | boolean | לא |  |

## תגובה

מחזיר: [`GetCommentsForUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUser200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getCommentsForUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const userId: string = "550e8400-e29b-41d4-a716-446655440000";
const page: number = 2;
const includei10n: boolean = true;
const locale: string = "en-US";
const isCrawler: boolean = false;

const comments: GetCommentsForUser200Response = await getCommentsForUser(
  userId,
  undefined, // direction הושמט
  undefined, // repliesToUserId הושמט
  page,
  includei10n,
  locale,
  isCrawler
);

console.log(comments);
[inline-code-end]