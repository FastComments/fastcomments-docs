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
const tenantId: string = "tenant_48d3a2b1";
const requestItem: BulkAggregateQuestionItem = {
  questionId: "q-7890",
  metric: "response_count",
  timeBuckets: [
    {
      start: "2025-01-01T00:00:00Z",
      end: "2025-01-07T23:59:59Z"
    }
  ]
};
const bulkAggregateQuestionResultsRequest: BulkAggregateQuestionResultsRequest = {
  items: [requestItem],
  includeTotals: true
};
const forceRecalculate: boolean = true;
const result: BulkAggregateQuestionResults200Response = await bulkAggregateQuestionResults(tenantId, bulkAggregateQuestionResultsRequest, forceRecalculate);
[inline-code-end]
