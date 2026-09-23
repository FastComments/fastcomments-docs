## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| questionId | string | לא |  |
| questionIds | Array<string> | לא |  |
| urlId | string | לא |  |
| startDate | Date | לא |  |
| forceRecalculate | boolean | לא |  |
| minValue | number | לא |  |
| maxValue | number | לא |  |
| limit | number | לא |  |

## תגובה

מחזיר: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CombineQuestionResultsWithCommentsResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה של combineCommentsWithQuestionResults'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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