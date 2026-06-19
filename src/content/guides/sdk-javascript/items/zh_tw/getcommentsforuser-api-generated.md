## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|------|------|
| userId | string | 否 |  |
| direction | SortDirections | 否 |  |
| repliesToUserId | string | 否 |  |
| page | number | 否 |  |
| includei10n | boolean | 否 |  |
| locale | string | 否 |  |
| isCrawler | boolean | 否 |  |

## 回應

回傳: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse.ts)

## 範例

[inline-code-attrs-start title = 'getCommentsForUser 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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