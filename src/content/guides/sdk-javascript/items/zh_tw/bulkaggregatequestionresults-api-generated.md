## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| bulkAggregateQuestionResultsRequest | BulkAggregateQuestionResultsRequest | 是 |  |
| forceRecalculate | boolean | 否 |  |

## 回應

返回: [`BulkAggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BulkAggregateQuestionResultsResponse.ts)

## 範例

[inline-code-attrs-start title = 'bulkAggregateQuestionResults 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";

const request: BulkAggregateQuestionResultsRequest = {
  items: [
    {
      questionId: "q_9876",
      startDate: "2023-01-01T00:00:00Z",
      endDate: "2023-01-31T23:59:59Z",
      timeBucket: "day"
    }
  ]
};

const response: BulkAggregateQuestionResultsResponse = await bulkAggregateQuestionResults(tenantId, request, true);
[inline-code-end]

---