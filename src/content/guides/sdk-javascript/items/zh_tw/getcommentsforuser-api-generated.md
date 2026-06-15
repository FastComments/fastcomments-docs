## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| userId | string | 否 |  |
| direction | SortDirections | 否 |  |
| repliesToUserId | string | 否 |  |
| page | number | 否 |  |
| includei10n | boolean | 否 |  |
| locale | string | 否 |  |
| isCrawler | boolean | 否 |  |

## 回應

回傳: [`GetCommentsForUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUser200Response.ts)

## 範例

[inline-code-attrs-start title = 'getCommentsForUser 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const userId: string = "550e8400-e29b-41d4-a716-446655440000";
const page: number = 2;
const includei10n: boolean = true;
const locale: string = "en-US";
const isCrawler: boolean = false;

const comments: GetCommentsForUser200Response = await getCommentsForUser(
  userId,
  undefined, // 已省略 direction
  undefined, // 已省略 repliesToUserId
  page,
  includei10n,
  locale,
  isCrawler
);

console.log(comments);
[inline-code-end]

---