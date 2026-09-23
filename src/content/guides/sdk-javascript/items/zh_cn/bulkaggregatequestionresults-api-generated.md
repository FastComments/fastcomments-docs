## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| bulkAggregateQuestionResultsRequest | BulkAggregateQuestionResultsRequest | Yes |  |
| forceRecalculate | boolean | No |  |

## 响应

返回：[`BulkAggregateQuestionResultsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BulkAggregateQuestionResultsResponse.ts)

## 示例

[inline-code-attrs-start title = 'bulkAggregateQuestionResults 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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