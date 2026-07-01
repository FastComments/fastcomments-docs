## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 响应

返回：[`GetQuestionResultResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResultResponse1.ts)

## 示例

[inline-code-attrs-start title = 'getQuestionResult 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchQuestionResult(): Promise<void> {
    const tenantId: string = "acme-corp-001";
    const questionId: string = "question-7a9b8c";
    const result: GetQuestionResultResponse1 = await getQuestionResult(tenantId, questionId);

    const question: QuestionResult | undefined = result.questionResult;
    const firstMeta: MetaItem | undefined = result.meta?.[0];

    console.log(question?.id, firstMeta?.key);
}
[inline-code-end]