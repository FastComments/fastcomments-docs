## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| value | string | 否 |  |
| filters | string | 否 |  |
| searchFilters | string | 否 |  |
| sso | string | 否 |  |

## 回應

回傳: [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationCommentSearchResponse.ts)

## 範例

[inline-code-attrs-start title = 'getSearchCommentsSummary 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const query: string = 'renewable energy incentives';
const filters: string = 'status:approved AND created_at>2025-01-01';
const searchFilters: string | undefined = undefined;
const sso: string | undefined = undefined;
const summary: ModerationCommentSearchResponse = await getSearchCommentsSummary(query, filters, searchFilters, sso);
[inline-code-end]

---