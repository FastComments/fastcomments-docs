## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| bulkAggregateQuestionResultsRequest | BulkAggregateQuestionResultsRequest | 예 |  |
| forceRecalculate | boolean | 아니오 |  |

## Response

반환: [`BulkAggregateQuestionResultsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BulkAggregateQuestionResultsResponse1.ts)

## Example

[inline-code-attrs-start title = 'bulkAggregateQuestionResults 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp";

const request: BulkAggregateQuestionResultsRequest = {
  questionIds: ["product-satisfaction", "feature-usage"],
  startDate: "2024-01-01T00:00:00Z",
  endDate: "2024-01-31T23:59:59Z",
  timeBucket: "day"
};

const result: BulkAggregateQuestionResultsResponse1 = await bulkAggregateQuestionResults(
  tenantId,
  request,
  true
);
[inline-code-end]

---