## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| userId | string | 아니요 |  |
| direction | SortDirections | 아니요 |  |
| repliesToUserId | string | 아니요 |  |
| page | number | 아니요 |  |
| includei10n | boolean | 아니요 |  |
| locale | string | 아니요 |  |
| isCrawler | boolean | 아니요 |  |

## 응답

반환: [`GetCommentsForUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsForUser200Response.ts)

## 예제

[inline-code-attrs-start title = 'getCommentsForUser 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const userId: string = "550e8400-e29b-41d4-a716-446655440000";
const page: number = 2;
const includei10n: boolean = true;
const locale: string = "en-US";
const isCrawler: boolean = false;

const comments: GetCommentsForUser200Response = await getCommentsForUser(
  userId,
  undefined, // direction omitted
  undefined, // repliesToUserId omitted
  page,
  includei10n,
  locale,
  isCrawler
);

console.log(comments);
[inline-code-end]

---