## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| bulkAggregateQuestionResultsRequest | BulkAggregateQuestionResultsRequest | 是 |  |
| forceRecalculate | boolean | 否 |  |

## 回應

回傳: [`BulkAggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BulkAggregateQuestionResultsResponse.ts)

## 範例

[inline-code-attrs-start title = 'bulkAggregateQuestionResults 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_3f9b2c1a';
const items: BulkAggregateQuestionItem[] = [
  {
    questionId: 'q_92f1b7',
    metrics: ['responses', 'upvotes'],
    timeBuckets: [
      { start: '2026-05-01T00:00:00Z', end: '2026-05-07T23:59:59Z', interval: 'day' }
    ]
  }
];
const bulkAggregateQuestionResultsRequest: BulkAggregateQuestionResultsRequest = { items };
const forceRecalculate: boolean = true;
const result: BulkAggregateQuestionResultsResponse = await bulkAggregateQuestionResults(tenantId, bulkAggregateQuestionResultsRequest, forceRecalculate);
[inline-code-end]