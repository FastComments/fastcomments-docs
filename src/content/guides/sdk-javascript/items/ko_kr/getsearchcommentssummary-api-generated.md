## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| value | string | 아니요 |  |
| filters | string | 아니요 |  |
| searchFilters | string | 아니요 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationCommentSearchResponse.ts)

## 예제

[inline-code-attrs-start title = 'getSearchCommentsSummary 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const query: string = 'renewable energy incentives';
const filters: string = 'status:approved AND created_at>2025-01-01';
const searchFilters: string | undefined = undefined;
const sso: string | undefined = undefined;
const summary: ModerationCommentSearchResponse = await getSearchCommentsSummary(query, filters, searchFilters, sso);
[inline-code-end]

---