## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| questionId | string | 아니오 |  |
| questionIds | Array<string> | 아니오 |  |
| urlId | string | 아니오 |  |
| startDate | Date | 아니오 |  |
| forceRecalculate | boolean | 아니오 |  |
| minValue | number | 아니오 |  |
| maxValue | number | 아니오 |  |
| limit | number | 아니오 |  |

## 응답

반환: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CombineQuestionResultsWithCommentsResponse.ts)

## 예제

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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