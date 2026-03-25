## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| bulkAggregateQuestionResultsRequest | BulkAggregateQuestionResultsRequest | 예 |  |
| forceRecalculate | boolean | 아니요 |  |

## 응답

반환: [`BulkAggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BulkAggregateQuestionResults200Response.ts)

## 예제

[inline-code-attrs-start title = 'bulkAggregateQuestionResults 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_live_7c9d";
const bulkAggregateQuestionResultsRequest: BulkAggregateQuestionResultsRequest = {
  items: [
    { questionId: "q_feedback_rating", includeSubQuestions: false, filters: { locale: "en-US" } }
  ],
  timeBuckets: [
    { granularity: "day", startUtc: "2026-03-01T00:00:00Z", endUtc: "2026-03-15T00:00:00Z" }
  ]
};
const forceRecalculate: boolean = true;
const result: BulkAggregateQuestionResults200Response = await bulkAggregateQuestionResults(tenantId, bulkAggregateQuestionResultsRequest, forceRecalculate);
[inline-code-end]