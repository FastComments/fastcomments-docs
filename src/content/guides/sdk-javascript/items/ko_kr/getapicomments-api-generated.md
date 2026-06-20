## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| page | number | 아니요 |  |
| count | number | 아니요 |  |
| textSearch | string | 아니요 |  |
| byIPFromComment | string | 아니요 |  |
| filters | string | 아니요 |  |
| searchFilters | string | 아니요 |  |
| sorts | string | 아니요 |  |
| demo | boolean | 아니요 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPIGetCommentsResponse.ts)

## 예제

[inline-code-attrs-start title = 'getApiComments 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const page: number = 2;
const count: number = 25;
const textSearch: string = 'comments failing to load after posting';
const filters: string = 'status:pending,moderation:required';
const sorts: string = 'createdAt:desc';
const demo: boolean = false;
const sso: string = 'sso-usr-7f3b2a';

const response: ModerationAPIGetCommentsResponse = await getApiComments(page, count, textSearch, undefined, filters, undefined, sorts, demo, sso);
[inline-code-end]

---