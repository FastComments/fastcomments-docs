## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| bulkAggregateQuestionResultsRequest | BulkAggregateQuestionResultsRequest | Yes |  |
| forceRecalculate | boolean | No |  |

## Response

Returns: [`BulkAggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BulkAggregateQuestionResults200Response.ts)

## Example

[inline-code-attrs-start title = 'bulkAggregateQuestionResults Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_acme_001';
  const bulkAggregateQuestionResultsRequest: BulkAggregateQuestionResultsRequest = {
    items: [
      {
        questionId: 'q_1001',
        buckets: [{ start: '2026-05-01T00:00:00Z', end: '2026-05-19T23:59:59Z' }]
      } as BulkAggregateQuestionItem
    ]
  };
  const forceRecalculate: boolean = true;
  const result: BulkAggregateQuestionResults200Response = await bulkAggregateQuestionResults(tenantId, bulkAggregateQuestionResultsRequest, forceRecalculate);
  console.log(result);
})();
[inline-code-end]
