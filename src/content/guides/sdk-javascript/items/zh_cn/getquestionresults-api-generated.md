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

返回：[`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResults200Response.ts)

## 示例

[inline-code-attrs-start title = 'getQuestionResults 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme_001";
const urlId: string = "articles/product-launch-2026";
const userId: string = "user_2048";
const startDate: string = "2026-03-01T00:00:00Z";
const questionId: string | undefined = undefined;
const questionIds: string | undefined = "q_101,q_102";
const skip: number | undefined = 0;

const result: GetQuestionResults200Response = await getQuestionResults(tenantId, urlId, userId, startDate, questionId, questionIds, skip);
[inline-code-end]

---