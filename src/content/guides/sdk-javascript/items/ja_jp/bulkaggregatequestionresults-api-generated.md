## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| bulkAggregateQuestionResultsRequest | BulkAggregateQuestionResultsRequest | はい |  |
| forceRecalculate | boolean | いいえ |  |

## レスポンス

戻り値: [`BulkAggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BulkAggregateQuestionResultsResponse.ts)

## 例

[inline-code-attrs-start title = 'bulkAggregateQuestionResults の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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