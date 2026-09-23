## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| questionId | string | No |  |
| questionIds | Array<string> | No |  |
| urlId | string | No |  |
| startDate | Date | No |  |
| forceRecalculate | boolean | No |  |
| minValue | number | No |  |
| maxValue | number | No |  |
| limit | number | No |  |

## 响应

返回：[`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CombineQuestionResultsWithCommentsResponse.ts)

## 示例

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = "tenant_12345";
  const questionId: string | undefined = "q_987";
  const questionIds: string[] = ["q_001", "q_002"];
  const urlId: string | undefined = "url_abc";
  const startDate: Date = new Date("2023-01-01T00:00:00Z");
  const forceRecalculate: boolean = true;
  const minValue: number = 0;
  const maxValue: number = 100;
  const limit: number = 50;

  const result: CombineQuestionResultsWithCommentsResponse = await combineCommentsWithQuestionResults(
    tenantId,
    questionId,
    questionIds,
    urlId,
    startDate,
    forceRecalculate,
    minValue,
    maxValue,
    limit
  );

  console.log(result);
}

runExample();
[inline-code-end]