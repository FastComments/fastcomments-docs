## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| userId | string | 아니오 |  |
| direction | SortDirections | 아니오 |  |
| repliesToUserId | string | 아니오 |  |
| page | number | 아니오 |  |
| includei10n | boolean | 아니오 |  |
| locale | string | 아니오 |  |
| isCrawler | boolean | 아니오 |  |

## 응답

반환: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUserResponse.ts)

## 예제

[inline-code-attrs-start title = 'getCommentsForUser 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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