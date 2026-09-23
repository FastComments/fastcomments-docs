## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| userId | string | No |  |
| direction | SortDirections | No |  |
| repliesToUserId | string | No |  |
| page | number | No |  |
| includei10n | boolean | No |  |
| locale | string | No |  |
| isCrawler | boolean | No |  |

## レスポンス

返り値: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse.ts)

## 例

[inline-code-attrs-start title = 'getCommentsForUser の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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