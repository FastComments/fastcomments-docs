## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| bulkAggregateQuestionResultsRequest | BulkAggregateQuestionResultsRequest | 是 |  |
| forceRecalculate | boolean | 否 |  |

## 回應

回傳：[`BulkAggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BulkAggregateQuestionResults200Response.ts)

## 範例

[inline-code-attrs-start title = 'bulkAggregateQuestionResults 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme_42";
const bulkAggregateQuestionResultsRequest: BulkAggregateQuestionResultsRequest = {
  questions: [
    { questionId: "q-001", threadId: "thread-1001", questionType: "rating" },
    { questionId: "q-002", threadId: "thread-1002", questionType: "yes_no" }
  ],
  timeRange: { from: "2026-03-01T00:00:00Z", to: "2026-04-01T00:00:00Z" },
  groupBy: ["questionId", "threadId"]
};
const forceRecalculate: boolean = true;
const result: BulkAggregateQuestionResults200Response = await bulkAggregateQuestionResults(tenantId, bulkAggregateQuestionResultsRequest, forceRecalculate);
[inline-code-end]

---