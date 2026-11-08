## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| bulkAggregateQuestionResultsRequest | BulkAggregateQuestionResultsRequest | हाँ |  |
| forceRecalculate | boolean | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`BulkAggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BulkAggregateQuestionResultsResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'bulkAggregateQuestionResults उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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