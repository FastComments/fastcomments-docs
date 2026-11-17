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
async function fetchQuestionResult(): Promise<void> {
  const tenantId: string = '5a3b8f9c2d1e4a0001aabbcc';
  const questionId: string = 'q-7892-hflu-survey';

  const response: GetQuestionResultResponse = await getQuestionResult(tenantId, questionId);

  const status: APIStatus = response.status;
  const result: QuestionResult | undefined = response.result;
  const meta: MetaItem[] | undefined = response.meta;

  if (result) {
    console.log(`Question ${result.id} status: ${status}`);
  }
}

fetchQuestionResult();
[inline-code-end]