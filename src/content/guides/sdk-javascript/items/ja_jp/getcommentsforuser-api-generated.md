---
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| userId | string | 任意 |  |
| direction | SortDirections | 任意 |  |
| repliesToUserId | string | 任意 |  |
| page | number | 任意 |  |
| includei10n | boolean | 任意 |  |
| locale | string | 任意 |  |
| isCrawler | boolean | 任意 |  |

## レスポンス

返却値: [`GetCommentsForUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUser200Response.ts)

## 例

[inline-code-attrs-start title = 'getCommentsForUser の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const userId: string = "550e8400-e29b-41d4-a716-446655440000";
const page: number = 2;
const includei10n: boolean = true;
const locale: string = "en-US";
const isCrawler: boolean = false;

const comments: GetCommentsForUser200Response = await getCommentsForUser(
  userId,
  undefined, // direction を省略
  undefined, // repliesToUserId を省略
  page,
  includei10n,
  locale,
  isCrawler
);

console.log(comments);
[inline-code-end]

---