## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| urlId | string | 否 |  |
| userId | string | 否 |  |
| startDate | string | 否 |  |
| questionId | string | 否 |  |
| questionIds | string | 否 |  |
| skip | number | 否 |  |

## 响应

返回：[`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResultsResponse.ts)

## 示例

[inline-code-attrs-start title = 'getQuestionResults 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_42f6';
const urlId: string = 'product-page-123';
const userId: string = 'user_789';
const startDate: string = '2024-05-01';
const questionIds: string = 'q-112,q-113';
const skip: number = 20;
const results: GetQuestionResultsResponse = await getQuestionResults(tenantId, urlId, userId, startDate, undefined, questionIds, skip);
[inline-code-end]

---