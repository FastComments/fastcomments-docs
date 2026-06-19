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

戻り値: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse.ts)

## 例

[inline-code-attrs-start title = 'getCommentsForUser の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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