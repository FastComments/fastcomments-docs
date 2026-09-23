## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 否 |  |
| userId | string | 否 |  |
| startDate | string | 否 |  |
| questionId | string | 否 |  |
| questionIds | string | 否 |  |
| skip | number | 否 |  |

## 响应

返回: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResultsResponse.ts)

## 示例

[inline-code-attrs-start title = 'getQuestionResults 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchResults() {
  const tenantId: string = 'c1a2b3d4-5678-90ab-cdef-1234567890ab';
  const urlId: string = 'blog-post-789';
  const startDate: string = '2023-01-01T00:00:00Z';
  const questionIds: string = 'q-abc123,q-def456';
  const response: GetQuestionResultsResponse = await getQuestionResults(
    tenantId,
    urlId,
    undefined,
    startDate,
    undefined,
    questionIds
  );
  console.log(response);
}
[inline-code-end]