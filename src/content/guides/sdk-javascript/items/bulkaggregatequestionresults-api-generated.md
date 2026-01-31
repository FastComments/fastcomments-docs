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
const tenantId: string = "tenant_live_8f3b2c1a";
const bulkAggregateQuestionResultsRequest: BulkAggregateQuestionResultsRequest = {
  questionIds: ["question-42", "question-99"],
  rangeStart: "2025-01-01T00:00:00Z",
  rangeEnd: "2025-01-31T23:59:59Z",
  buckets: 31
};
const forceRecalculate: boolean = true;
const result: BulkAggregateQuestionResults200Response = await bulkAggregateQuestionResults(tenantId, bulkAggregateQuestionResultsRequest, forceRecalculate);
[inline-code-end]
