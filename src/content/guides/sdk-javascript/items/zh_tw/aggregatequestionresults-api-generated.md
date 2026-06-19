---
## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| questionId | string | 否 |  |
| questionIds | Array<string> | 否 |  |
| urlId | string | 否 |  |
| timeBucket | AggregateTimeBucket | 否 |  |
| startDate | Date | 否 |  |
| forceRecalculate | boolean | 否 |  |

## 回應

回傳：[`AggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/AggregateQuestionResultsResponse.ts)

## 範例

[inline-code-attrs-start title = 'aggregateQuestionResults 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8a4f7c2b';
const questionIds: string[] = ['q_101', 'q_102', 'q_103'];
const urlId: string = 'url_3f9b12';
const startDate: Date = new Date('2026-01-01T00:00:00Z');
const forceRecalculate: boolean = true;
const result: AggregateQuestionResultsResponse = await aggregateQuestionResults(tenantId, undefined, questionIds, urlId, undefined, startDate, forceRecalculate);
[inline-code-end]

---