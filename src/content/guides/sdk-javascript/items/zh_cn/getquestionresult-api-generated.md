## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 响应

返回：[`GetQuestionResultResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResultResponse.ts)

## 示例

[inline-code-attrs-start title = 'getQuestionResult 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const questionId: string = "question_98765";

const response: GetQuestionResultResponse = await getQuestionResult(tenantId, questionId);

const status: APIStatus | undefined = response.status;
const firstResult: QuestionResult | undefined = response.results?.[0];
const firstMeta: MetaItem | undefined = response.meta?.[0];

console.log(response);
[inline-code-end]