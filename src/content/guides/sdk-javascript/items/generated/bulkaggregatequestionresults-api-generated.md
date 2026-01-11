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
  const tenantId: string = "tenant_prod_84b2";
  const bulkAggregateQuestionResultsRequest: BulkAggregateQuestionResultsRequest = {
    questionIds: ["q_9fa1b2", "q_3c4d5e"],
    startDate: "2025-10-01T00:00:00Z",
    endDate: "2025-10-31T23:59:59Z",
    groupBy: "question",
    timeBucket: "day"
  };
  const forceRecalculate: boolean = true;
  const result: BulkAggregateQuestionResults200Response = await bulkAggregateQuestionResults(tenantId, bulkAggregateQuestionResultsRequest, forceRecalculate);
  console.log(result);
})();
[inline-code-end]
